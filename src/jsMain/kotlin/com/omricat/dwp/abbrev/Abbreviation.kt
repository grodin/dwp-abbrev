package com.omricat.dwp.abbrev

internal data class Abbreviation(
  val abbreviation: String,
  val description: String
) {
  companion object {
    fun fromPair(pair: Pair<String, String>): Abbreviation =
      Abbreviation(abbreviation = pair.first, description = pair.second)
  }
}

internal fun Collection<Pair<String, String>>.toAbbreviationList(): List<Abbreviation> =
  map { Abbreviation.fromPair(it) }
