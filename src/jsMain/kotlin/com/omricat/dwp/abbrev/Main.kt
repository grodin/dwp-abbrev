package com.omricat.dwp.abbrev

import androidx.compose.runtime.getValue
import androidx.compose.runtime.mutableStateOf
import androidx.compose.runtime.remember
import androidx.compose.runtime.setValue
import org.jetbrains.compose.web.css.padding
import org.jetbrains.compose.web.css.px
import org.jetbrains.compose.web.dom.*
import org.jetbrains.compose.web.renderComposable

fun main() {

  renderComposable(rootElementId = "root") {

    var filter: String by remember { mutableStateOf("") }

    Div({ style { padding(25.px) } }) {
      TextInput {
        value(filter)
        onInput {
          filter = it.value
        }
      }
    }

    Table {
      dummyAbbrevs.filter { abbrev ->
        if (filter.isBlank()) true
        else abbrev.abbreviation.startsWith(filter.trim(), ignoreCase = true)
      }.forEach {
        Tr {
          Td { Text(it.abbreviation) }
          Td { Text(it.description) }
        }
      }
    }

  }
}

internal val dummyAbbrevs = abbreviationsFromPairs(
  listOf(
    "AB" to "Arse Biscuits",
    "CB" to "Click Biscuits",
    "DWP" to "Department for Work and Pensions",
  )
)
