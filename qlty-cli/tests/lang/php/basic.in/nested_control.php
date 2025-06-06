<?php
  function not_nested($foo, $bar) {
    if (($foo === 'cat' && $bar === 'dog') || ($foo === 'dog' && $bar === 'cat')) {
      echo 'Got a cat and a dog!';
    } else {
      echo 'Got nothing';
    }
  }

  function f0() {
    if ($bar) {
      if ($baz) {
        if ($qux) {
          if ($quux) {
            echo 'Not deeply nested enough!';
          }
        }
      }
    }
  }

  function f1() {
    if ($bar) {
      if ($baz) {
        if ($qux) {
          if ($quux) {
            echo 'Deeply nested!';
          }
        }
      }
    }
  }

  function f2($foo) {
    switch ($foo) {
      case 1:
        return 'bar1';
      case 2:
        return 'bar2';
      case 3:
        return 'bar3';
      case 4:
        return 'bar4';
      case 5:
        return 'bar5';
      case 6:
        return 'bar6';
      case 7:
        return 'bar7';
      case 8:
        return 'bar8';
      case 9:
        return 'bar9';
      case 10:
        return 'bar10';
      default:
        throw new Exception('Invalid foo value');
    }
  }
?>
