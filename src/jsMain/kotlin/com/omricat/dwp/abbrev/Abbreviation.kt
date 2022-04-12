package com.omricat.dwp.abbrev

internal data class Abbreviation(val abbreviation: String, val description: String) {
  companion object {
    fun fromPair(pair: Pair<String, String>): Abbreviation =
      Abbreviation(abbreviation = pair.first, description = pair.second)
  }
}

internal fun abbreviationsFromPairs(
  pairs: Collection<Pair<String, String>>
): List<Abbreviation> = pairs.map { Abbreviation.fromPair(it) }
