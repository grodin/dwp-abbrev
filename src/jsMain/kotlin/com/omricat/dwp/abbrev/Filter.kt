package com.omricat.dwp.abbrev

internal fun filter(filterQuery: String, item: String): Boolean {
    val alphaNumericOnly = item.trim().replace("""[^A-Za-z0-9]+""".toRegex(), "")
    val alphaNumNoConnectives = 
    	connectives.fold(alphaNumericOnly) {s, c -> s.replace(c, "") }
    fun matches(s: String): Boolean = 
    	s.startsWith(filterQuery.trim(), ignoreCase=true)
    
    return matches(alphaNumericOnly) || matches(alphaNumNoConnectives)
}

private val connectives = listOf("and", "to", "or")
