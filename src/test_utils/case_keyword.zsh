foo() {
  case $# in
    0) echo "zero args" ;;
    1) echo "one arg" ;;
    \#) echo "hash arg" ;;
    *) echo "many args" ;;
  esac
}

foo
