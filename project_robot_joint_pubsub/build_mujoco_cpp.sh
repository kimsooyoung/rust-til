#!/bin/bash
# Build script for MuJoCo with C++ viewer support for mujoco-rs
# This builds the modified MuJoCo library required for cpp-viewer feature

set -e

echo "🔨 Building MuJoCo with C++ viewer support for mujoco-rs"
echo "=========================================================="

# Check if mujoco-rs repo exists, if not clone it
# Note:
# - Some environments don't have SSH access to GitHub (or have broken ssh url rewrites).
# - mujoco-rs uses an SSH submodule URL for `mujoco` in `.gitmodules`.
# - To keep this script robust, we force HTTPS just for the submodule commands via `git -c ...`,
#   without requiring any global git config changes.
MUJOCO_RS_REPO_DIR="${MUJOCO_RS_REPO_DIR:-../mujoco-rs}"
MUJOCO_BUILD_DIR="${MUJOCO_BUILD_DIR:-mujoco_build}"

if [ ! -d "$MUJOCO_RS_REPO_DIR" ]; then
    echo "📦 Cloning mujoco-rs repository..."
    git clone https://github.com/davidhozic/mujoco-rs.git "$MUJOCO_RS_REPO_DIR"
else
    echo "✅ mujoco-rs repository found at $MUJOCO_RS_REPO_DIR"
fi

echo "🔄 Initializing / updating submodules (forcing HTTPS)..."
# Force the `mujoco` submodule URL to HTTPS for this command invocation.
# This avoids reliance on any existing (possibly broken) url rewrite rules in user/system git config.
git -C "$MUJOCO_RS_REPO_DIR" \
    -c submodule.mujoco.url=https://github.com/davidhozic/mujoco.git \
    submodule update --init --recursive

# Navigate to mujoco submodule
MUJOCO_DIR="$MUJOCO_RS_REPO_DIR/mujoco"
if [ ! -d "$MUJOCO_DIR" ]; then
    echo "❌ Error: mujoco submodule not found at $MUJOCO_DIR"
    exit 1
fi

echo "📂 Building MuJoCo from $MUJOCO_DIR"

# Create build directory
cd "$MUJOCO_DIR"
mkdir -p build
cd build

# Configure CMake for static build with libsimulate
echo "⚙️  Configuring CMake..."
cmake \
    -DBUILD_SHARED_LIBS=OFF \
    -DMUJOCO_HARDEN=OFF \
    -DCMAKE_BUILD_TYPE=Release \
    -DMUJO_BUILD_EXAMPLES=OFF \
    -DCMAKE_EXE_LINKER_FLAGS="-Wl,--no-as-needed" \
    ..

# Build libsimulate target
echo "🔨 Building libsimulate..."
cmake --build . --parallel --target libsimulate

# Find the built library
LIB_DIR=$(pwd)
if [ -f "$LIB_DIR/libsimulate.a" ]; then
    echo "✅ Static library built: $LIB_DIR/libsimulate.a"
elif [ -f "$LIB_DIR/libsimulate.so" ]; then
    echo "✅ Shared library built: $LIB_DIR/libsimulate.so"
else
    echo "⚠️  Warning: Could not find libsimulate library"
fi

# Also build the main mujoco library if needed
echo "🔨 Building mujoco library..."
cmake --build . --parallel --target mujoco

# Check for mujoco library
if [ -f "$LIB_DIR/libmujoco.a" ]; then
    echo "✅ Static library built: $LIB_DIR/libmujoco.a"
elif [ -f "$LIB_DIR/libmujoco.so" ]; then
    echo "✅ Shared library built: $LIB_DIR/libmujoco.so"
fi

echo ""
echo "✅ Build complete!"
echo ""
echo "📝 Next steps:"
echo "1. Set MUJOCO_STATIC_LINK_DIR environment variable:"
echo "   export MUJOCO_STATIC_LINK_DIR=\"$LIB_DIR\""
echo ""
echo "2. Update your Cargo.toml to use cpp-viewer feature:"
echo "   mujoco-rs = { version = \"2.2.2\", features = [\"cpp-viewer\", ...] }"
echo ""
echo "3. Remove auto-download-mujoco feature (not compatible with static linking)"
echo ""
echo "Library directory: $LIB_DIR"
