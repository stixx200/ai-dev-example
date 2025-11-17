#!/bin/bash

# Workspace Setup Verification Script
# This script verifies that the Rust monorepo is set up correctly

set -e

echo "🔍 Verifying Rust Monorepo Setup..."
echo "======================================"
echo ""

# Check NX installation
echo "✓ Checking NX installation..."
if command -v npx &> /dev/null; then
    echo "  NX is available via npx"
else
    echo "  ❌ npx not found. Please install Node.js"
    exit 1
fi

# Check Rust installation
echo "✓ Checking Rust installation..."
if command -v cargo &> /dev/null; then
    RUST_VERSION=$(cargo --version)
    echo "  $RUST_VERSION"
else
    echo "  ❌ Cargo not found. Please install Rust from https://rustup.rs"
    exit 1
fi

# List all projects
echo ""
echo "✓ Listing all projects..."
PROJECTS=$(npx nx show projects)
echo "$PROJECTS" | sed 's/^/  - /'

# Verify project structure
echo ""
echo "✓ Verifying project structure..."
for project in my_app core data utils; do
    if npx nx show project $project &> /dev/null; then
        echo "  ✓ $project exists"
    else
        echo "  ❌ $project not found"
        exit 1
    fi
done

# Check GitHub Actions workflow
echo ""
echo "✓ Checking CI/CD setup..."
if [ -f ".github/workflows/ci.yml" ]; then
    echo "  ✓ GitHub Actions workflow exists"
else
    echo "  ❌ CI workflow not found"
    exit 1
fi

# Run tests
echo ""
echo "✓ Running all tests..."
if npx nx run-many -t test --skip-nx-cache 2>&1 | grep -q "Successfully ran target test"; then
    echo "  ✓ All tests passed"
else
    echo "  ❌ Tests failed"
    exit 1
fi

# Build all projects
echo ""
echo "✓ Building all projects..."
if npx nx run-many -t build --skip-nx-cache 2>&1 | grep -q "Successfully ran target build"; then
    echo "  ✓ All projects built successfully"
else
    echo "  ❌ Build failed"
    exit 1
fi

# Lint all projects
echo ""
echo "✓ Linting all projects..."
if npx nx run-many -t lint --skip-nx-cache 2>&1 | grep -q "Successfully ran target lint"; then
    echo "  ✓ All projects passed linting"
else
    echo "  ❌ Linting failed"
    exit 1
fi

# Run the application
echo ""
echo "✓ Running the application..."
if npx nx run my_app 2>&1 | grep -q "Application completed successfully"; then
    echo "  ✓ Application runs successfully"
else
    echo "  ❌ Application failed to run"
    exit 1
fi

echo ""
echo "======================================"
echo "✅ All verifications passed!"
echo ""
echo "Your Rust monorepo is set up correctly and ready to use."
echo ""
echo "Next steps:"
echo "  - Run 'npx nx graph' to visualize project dependencies"
echo "  - Run 'npx nx run my_app' to execute the application"
echo "  - Check WORKSPACE_OVERVIEW.md for detailed documentation"
echo ""
