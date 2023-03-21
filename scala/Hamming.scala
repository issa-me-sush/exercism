object Hamming {
  def distance(dnaStrandOne: String, dnaStrandTwo: String): Option[Int] = {
    if (dnaStrandOne.length != dnaStrandTwo.length) {
      // Sequences are of different lengths, return None
      None
    } else {
      // Calculate the Hamming distance by comparing each character
      // at the same position in both sequences
      Some((dnaStrandOne zip dnaStrandTwo).count{ case (a, b) => a != b })
    }
  }
}
