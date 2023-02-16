object Leap {
  def leapYear(year: Int): Boolean = {
     val a = year % 4 
     val b = year % 100
     val c = year % 400
     val d = (a == 0)
    if(d){
      if(c==0){
        return true
      }
      else if(b==0){
        return false
      }
      else {
        return true
      }
    
    }
    else {return false}  // dont forget to include the default return for the else case too, else throws type mismatch
      
  }
}
