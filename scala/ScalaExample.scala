object Main extends App {
  println("Hello, World!")
  println(twofer("yo"))
  def twofer(name: String = "you"): String =
  s"One for $name, one for me."
}