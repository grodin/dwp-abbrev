package com.omricat.dwp.abbrev

import androidx.compose.runtime.getValue
import androidx.compose.runtime.mutableStateOf
import androidx.compose.runtime.remember
import androidx.compose.runtime.setValue
import org.jetbrains.compose.web.attributes.autoFocus
import org.jetbrains.compose.web.css.padding
import org.jetbrains.compose.web.css.px
import org.jetbrains.compose.web.dom.*
import org.jetbrains.compose.web.renderComposable

private const val MAX_ENTRIES_SHOWN = 100

fun main() {

  renderComposable(rootElementId = "root") {

    var filterQuery: String by remember { mutableStateOf("") }

    Div({ style { padding(25.px) } }) {
      TextInput {
        autoFocus()
        value(filterQuery)
        onInput {
          filterQuery = it.value
        }
      }
    }

    Table {
      abbreviations.filter { abbrev ->
        filter(filterQuery, abbrev.abbreviation)
      }.take(MAX_ENTRIES_SHOWN)
      .sortedBy { it.abbreviation }
      .forEach {
        Tr {
          Td { Text(it.abbreviation) }
          Td { Text(it.description) }
        }
      }
    }

  }
}


