foo() { local bar=$1; echo $bar; }; foo "hello world"; qux() ( echo "${1} my ${2}"; ); qux "hello" "world"
