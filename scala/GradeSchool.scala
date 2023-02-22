class School {
  type DB = Map[Int, Seq[String]]

  private var roster: DB = Map.empty

  def add(name: String, grade: Int): Unit = {
    val students = roster.getOrElse(grade, Seq.empty)
    roster += (grade -> (students :+ name).sorted)
  }

  def db: DB = roster

  def grade(grade: Int): Seq[String] = roster.getOrElse(grade, Seq.empty)

  def sorted: DB = roster.toSeq.sortBy(_._1).map { case (grade, students) => (grade, students.sorted) }.toMap
}

