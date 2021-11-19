// SPDX-License-Identifier: MIT OR Apache-2.0
//
// Copyright (c) 2021 Andre Richter <andre.o.richter@gmail.com>


//--------------------------------------------------------------------------------------------------
// Notes on bootloader
//--------------------------------------------------------------------------------------------------

// The bootloader passes us a pointer to the DTB file in x0 so remember not to clobber it & pass it to
// the rust code to parse for memory map

//--------------------------------------------------------------------------------------------------
// Definitions
//--------------------------------------------------------------------------------------------------

// Load the address of a symbol into a register, PC-relative.
//
// The symbol must lie within +/- 4 GiB of the Program Counter.
//
// # Resources
//
// - https://sourceware.org/binutils/docs-2.36/as/AArch64_002dRelocations.html
.macro ADR_REL register, symbol
	adrp	\register, \symbol
	add		\register, \register, #:lo12:\symbol
.endm

.equ _core_id_mask, 0b11
.equ _EL2, 0x8

//--------------------------------------------------------------------------------------------------
// Public Code
//--------------------------------------------------------------------------------------------------
.section .text._start

//------------------------------------------------------------------------------
// fn _start()
//------------------------------------------------------------------------------
_start:
	// move the DTB pointer into x5 for safekeeping for now
	mov x5, x0

	// Only start in EL2 otherwise park the core
	mrs x0, CurrentEL
	cmp x0, _EL2
	b.ne .park_loop

	// Only proceed on the boot core. Park it otherwise.
	mrs	x1, MPIDR_EL1
	and	x1, x1, _core_id_mask
	ldr	x2, BOOT_CORE_ID      // provided by pi/cpu.rs rexported in arch/boot.rs
	cmp	x1, x2
	b.ne	.park_loop

	// If execution reaches here, it is the boot core.

	// Initialize DRAM to zero bss.
	ADR_REL	x0, __bss_start
	ADR_REL x1, __bss_end_exclusive

.L_bss_init_loop:
	cmp	x0, x1
	b.eq	.L_prepare_rust
	stp	xzr, xzr, [x0], #16
	b	.L_bss_init_loop

.L_prepare_rust:
	// Set the stack pointer.
	ADR_REL	x0, __boot_core_stack_end_exclusive
	mov	sp, x0

	// move DTB pointer into x1 to pass to rust
	mov x1, x5

	//Jump to Rust.
	// x0: Stack pointer
	// x1: DTB pointer
	b	_start_rust


	// Infinitely wait for events (aka "park the core").
.park_loop:	
	wfe
	b	.park_loop

.size	_start, . - _start
.type	_start, function
.global	_start