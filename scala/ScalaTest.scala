object ScalaTest{  
    def main(args: Array[String]) : Unit ={  
        println ("Hello Scala"  )
        println(args.mkString("\n"))
        println(args.mkString(
            start="Array(",
            sep  = ",",
            end = ")"

        ))                       //Scala ScalaTest.scala arg1 arg2 arg3..

        //keywords - val , var, def, lazy, type 
        val a = {        // val is similar to const in js
            println("binding 42 to a")  // just a statement

            // 24   //throws a  warning

            42  //last value is bound
        } 

        println(a)

        var b = {  // var is similar to let  in js
              println("let is mutable")

              420

        }

        println(b)

        b+=1

        println(b)

        def c =  {  // similar to defining a function
           println("testing the function")

        //    124  //throws a warning

           println("end of function " + b)

           125 // last non - statement gets printed too  - no warning here
        }
        println(c)
        println(c)

        lazy val  d = {
            println("val test")

            156
        }

        println(d)
        println(d)

        //top level defintions - functions 
        

    }  
}  