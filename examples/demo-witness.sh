#!/bin/bash
# Witness Demo Script
# Demonstrates selective disclosure of C2PA manifests using zero-knowledge proofs

set -e  # Exit on error

# Use rustup's Rust toolchain (1.92.0) which supports c2pa crate
export PATH="$HOME/.cargo/bin:$PATH"

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
CYAN='\033[0;36m'
NC='\033[0m' # No Color

# Configuration
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(cd "$SCRIPT_DIR/.." && pwd)"
C2PA_IMAGE="$PROJECT_ROOT/examples/c2pa/C.jpg"
SPEC_FILE="$SCRIPT_DIR/demo/witness-spec.json"
OUTPUT_VCE="$SCRIPT_DIR/demo/output.vce"
TEMP_FULL_MANIFEST=$(mktemp)
TEMP_REDACTED_OUTPUT=$(mktemp)

# Cleanup function
cleanup() {
    rm -f "$TEMP_FULL_MANIFEST" "$TEMP_REDACTED_OUTPUT"
}
trap cleanup EXIT

echo -e "${CYAN}═══════════════════════════════════════════════════════════════${NC}"
echo -e "${CYAN}           Witness: Selective Disclosure Demo${NC}"
echo -e "${CYAN}═══════════════════════════════════════════════════════════════${NC}"
echo ""

# Check prerequisites
echo -e "${BLUE}📋 Checking prerequisites...${NC}"
if [ ! -f "$C2PA_IMAGE" ]; then
    echo -e "${RED}❌ Error: C2PA image not found at $C2PA_IMAGE${NC}"
    exit 1
fi

if [ ! -f "$SPEC_FILE" ]; then
    echo -e "${RED}❌ Error: Spec file not found at $SPEC_FILE${NC}"
    exit 1
fi

echo -e "${GREEN}✓ C2PA image found${NC}"
echo -e "${GREEN}✓ Spec file found${NC}"
echo ""

# Step 1: Extract full manifest
echo -e "${BLUE}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
echo -e "${BLUE}Step 1: Extracting Full C2PA Manifest${NC}"
echo -e "${BLUE}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
echo ""

cd "$PROJECT_ROOT"
# Try to extract using the helper binary
if cargo run --release --bin extract-c2pa-claim -- --input "$C2PA_IMAGE" > "$TEMP_FULL_MANIFEST" 2>&1; then
    echo -e "${GREEN}✓ Full manifest extracted${NC}"
elif cargo run --release --bin inspect-c2pa -- --input "$C2PA_IMAGE" > /dev/null 2>&1; then
    # Fallback: Use inspect-c2pa and note that full JSON extraction requires Rust 1.88+
    echo -e "${YELLOW}⚠ Note: Full JSON extraction requires Rust 1.88+.${NC}"
    echo -e "${YELLOW}   Using simplified manifest view.${NC}"
    echo -e "${YELLOW}   (The demo will still show selective disclosure working)${NC}"
    echo '{"note": "Full manifest extraction requires Rust 1.88+. The selective disclosure demo will still work correctly."}' > "$TEMP_FULL_MANIFEST"
else
    echo -e "${RED}❌ Failed to extract C2PA manifest${NC}"
    echo -e "${YELLOW}   This may be due to Rust version requirements (c2pa crate needs Rust 1.88+)${NC}"
    echo -e "${YELLOW}   The demo can still proceed, but full manifest comparison will be limited.${NC}"
    echo '{"note": "Manifest extraction unavailable. Selective disclosure demo will still work."}' > "$TEMP_FULL_MANIFEST"
fi
echo ""

# Step 2: Generate VCE with selective disclosure
echo -e "${BLUE}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
echo -e "${BLUE}Step 2: Generating VCE with Selective Disclosure${NC}"
echo -e "${BLUE}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
echo ""
echo -e "${YELLOW}ℹ️  This will generate a zero-knowledge proof. This may take several minutes.${NC}"
echo ""

cd "$PROJECT_ROOT"
cargo run --release --bin fuse-prove -- \
    --spec "$SPEC_FILE" \
    --system "$C2PA_IMAGE" \
    --output "$OUTPUT_VCE" 2>&1 | tee /tmp/fuse-prove-output.log || {
    echo -e "${RED}❌ Failed to generate VCE${NC}"
    exit 1
}

echo ""
echo -e "${GREEN}✓ VCE generated successfully${NC}"
echo ""

# Step 3: Verify VCE and capture redacted output
echo -e "${BLUE}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
echo -e "${BLUE}Step 3: Verifying VCE and Extracting Redacted Manifest${NC}"
echo -e "${BLUE}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
echo ""

cd "$PROJECT_ROOT"
cargo run --release --bin fuse-verify -- "$OUTPUT_VCE" > "$TEMP_REDACTED_OUTPUT" 2>&1 || {
    echo -e "${RED}❌ Failed to verify VCE${NC}"
    cat "$TEMP_REDACTED_OUTPUT"
    exit 1
}

# Extract redacted JSON and claim hash from verify output
# Get everything after "Redacted Manifest Data:" until the next section
REDACTED_JSON=$(awk '/Redacted Manifest Data:/{flag=1; next} /^[[:space:]]*$/{if(flag && prev!="") exit} {if(flag) prev=$0; if(flag && prev!="") print prev}' "$TEMP_REDACTED_OUTPUT" 2>/dev/null || \
    grep -A 100 "Redacted Manifest Data:" "$TEMP_REDACTED_OUTPUT" | grep -v "Redacted Manifest Data:" | sed '/^$/d' | sed '/^[[:space:]]*$/d' | head -n -1 || echo "")
CLAIM_HASH=$(grep "Original Claim Hash" "$TEMP_REDACTED_OUTPUT" | awk '{print $NF}' || echo "")

echo -e "${GREEN}✓ VCE verified successfully${NC}"
echo ""

# Step 4: Display side-by-side comparison
echo -e "${BLUE}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
echo -e "${BLUE}Step 4: Comparison - Full vs. Redacted Manifest${NC}"
echo -e "${BLUE}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
echo ""

echo -e "${CYAN}═══════════════════════════════════════════════════════════════${NC}"
echo -e "${CYAN}FULL MANIFEST (All Fields)${NC}"
echo -e "${CYAN}═══════════════════════════════════════════════════════════════${NC}"
cat "$TEMP_FULL_MANIFEST"
echo ""
echo ""

echo -e "${CYAN}═══════════════════════════════════════════════════════════════${NC}"
echo -e "${CYAN}REDACTED MANIFEST (Only Disclosed Fields)${NC}"
echo -e "${CYAN}═══════════════════════════════════════════════════════════════${NC}"
if [ -n "$REDACTED_JSON" ]; then
    echo "$REDACTED_JSON"
else
    echo -e "${YELLOW}(No redacted JSON found in output)${NC}"
fi
echo ""
echo ""

if [ -n "$CLAIM_HASH" ]; then
    echo -e "${CYAN}═══════════════════════════════════════════════════════════════${NC}"
    echo -e "${CYAN}CRYPTOGRAPHIC BINDING${NC}"
    echo -e "${CYAN}═══════════════════════════════════════════════════════════════${NC}"
    echo -e "Original Claim Hash (SHA256): ${GREEN}$CLAIM_HASH${NC}"
    echo ""
    echo -e "${YELLOW}ℹ️  This hash cryptographically binds the redacted output to the original signed manifest.${NC}"
    echo ""
fi

# Step 5: Value proposition summary
echo -e "${BLUE}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
echo -e "${BLUE}Value Proposition Summary${NC}"
echo -e "${BLUE}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
echo ""

echo -e "${GREEN}✓ What Was Proven:${NC}"
echo "  • The full C2PA manifest was cryptographically verified"
echo "  • The signature was validated using zero-knowledge proof"
echo "  • The verification is publicly verifiable without trusted parties"
echo ""

echo -e "${GREEN}✓ What Was Hidden:${NC}"
DISCLOSED_FIELDS=$(grep -A 3 '"disclosed_fields"' "$SPEC_FILE" | grep -v '"disclosed_fields"' | grep -v '\[' | grep -v '\]' | sed 's/.*"\(.*\)".*/\1/' | tr '\n' ', ' | sed 's/,$//')
echo "  • Only these fields were disclosed: ${CYAN}$DISCLOSED_FIELDS${NC}"
echo "  • All other fields remain private"
echo "  • The verifier cannot see hidden data, only proof of verification"
echo ""

echo -e "${GREEN}✓ Cryptographic Guarantees:${NC}"
echo "  • The redacted output is cryptographically bound to the original manifest"
echo "  • The proof cannot be forged or tampered with"
echo "  • Anyone can verify the proof without access to the original image"
echo ""

echo -e "${CYAN}═══════════════════════════════════════════════════════════════${NC}"
echo -e "${GREEN}✅ Demo Complete!${NC}"
echo -e "${CYAN}═══════════════════════════════════════════════════════════════${NC}"
echo ""
echo -e "Generated VCE file: ${CYAN}$OUTPUT_VCE${NC}"
echo ""

