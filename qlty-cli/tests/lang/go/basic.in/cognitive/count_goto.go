package foo

import "fmt"

func foo() {
	fmt.Println(1)
	goto End
	fmt.Println(2)
End:
	fmt.Println(3)
}
