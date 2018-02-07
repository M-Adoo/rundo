echo "deploy types"
cd types
cargo package 
cargo publish
echo "deploy attrs"
cd ../macros/attrs
cargo package 
cargo publish
echo "deploy rundo"
git status
cd ../../
cargo package 
cargo publish