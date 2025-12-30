# Ed25519 Compatibility Issue Analysis

## The Problem

**Error:** `Trap: IllegalInstruction(0330000f), pc: 0x00009266`

**What's happening:**
- `ed25519-compact` v2.2.0 compiles successfully for `riscv32im-risc0-zkvm-elf`
- But at runtime, it executes an instruction that RISC Zero zkVM doesn't support
- This happens immediately when Ed25519 verification code runs

**Why this matters:**
- We can't measure Ed25519 performance overhead
- We can't proceed with C2PA pivot
- This is a fundamental technical blocker

## Root Cause

The `ed25519-compact` crate likely uses:
1. **Unsupported RISC-V instructions** - May use instructions not in the `riscv32im` subset
2. **Floating point operations** - Not available in `riscv32im` (no FPU)
3. **Atomic operations** - May use atomics incompatible with zkVM
4. **CPU-specific optimizations** - May have platform-specific code paths

## What We Need

### Option 1: Try Alternative Ed25519 Crates (Recommended First)

**Try `ed25519-dalek` with `no_std`:**
```toml
ed25519-dalek = { version = "2.0", default-features = false, features = ["no_std"] }
```

**Why this might work:**
- Pure Rust implementation
- Explicit `no_std` support
- Widely used in embedded/zkVM contexts
- May have better RISC-V compatibility

### Option 2: Check RISC Zero Built-in Crypto

RISC Zero may provide:
- Built-in Ed25519 support
- Crypto primitives optimized for zkVM
- Examples in RISC Zero documentation

**Action:** Check RISC Zero docs/examples for Ed25519 support

### Option 3: Investigate the Specific Instruction

**The failing instruction:** `0330000f` at `pc: 0x00009266`

**What we need:**
- Disassemble the guest program binary
- Identify what instruction `0330000f` represents
- Determine if it's a known incompatibility
- See if we can work around it

**Tools needed:**
- RISC-V disassembler (`riscv64-unknown-elf-objdump` or similar)
- RISC Zero debugging tools

### Option 4: Alternative Architecture

**If Ed25519 doesn't work:**
- Use host-side verification with selective disclosure
- Consider alternative signature schemes (if C2PA allows)
- Implement minimal Ed25519 using only supported operations

## Immediate Next Steps

1. **Try `ed25519-dalek`** (highest probability of success)
   - Update `fuse-guest/Cargo.toml`
   - Rebuild guest program
   - Test in dev mode

2. **Check RISC Zero examples**
   - Look for Ed25519 examples in RISC Zero repo/docs
   - Check if they provide crypto primitives

3. **If both fail:**
   - Document findings
   - Consider alternative approaches
   - Reassess pivot strategy

## Current Status

- ✅ Baseline works: 23.80 minutes
- ❌ Ed25519 fails: IllegalInstruction
- ⏳ Need to test alternatives

