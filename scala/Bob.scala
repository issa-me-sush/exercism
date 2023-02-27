object Bob {
  def response(statement: String): String = {
    val trimmed = statement.trim() // Remove leading/trailing whitespace
    if (trimmed.isEmpty()) {
      "Fine. Be that way!"
    } else if (trimmed.endsWith("?")) {
      if (trimmed == trimmed.toUpperCase() && trimmed.exists(_.isLetter)) {
        "Calm down, I know what I'm doing!"
      } else {
        "Sure."
      }
    } else if (trimmed == trimmed.toUpperCase() && trimmed.exists(_.isLetter)) {
      "Whoa, chill out!"
    } else {
      "Whatever."
    }
  }
}
