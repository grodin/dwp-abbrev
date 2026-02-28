import abbreviations from "./abbreviations.json" with {type: "json"}
import {matchFilterAgainstQuery} from "./abbreviation-filter.ts";

const filterQueryInput = document.getElementById('filter-query') as HTMLInputElement;

const abbreviationsTable = document.getElementById("abbreviations-table") as HTMLTableElement;

window.onfocus = () => {
    filterQueryInput.focus()
    filterQueryInput.select()
}

window.onkeydown = (event) => {
    if (event.key === "Escape") {
        filterQueryInput.value = ""
        filterQueryInput.focus()
        updateFilteredAbbreviations(filterQueryInput.value)
    }
}

filterQueryInput.onfocus = () => {
    filterQueryInput.select()
}

function updateFilteredAbbreviations(filterQuery: string) {
    const filteredAbbrevs = abbreviations
        .map((entry) => matchFilterAgainstQuery(filterQuery, entry))
        .filter((entry) => entry !== null)
        .slice(0, 150)
        .sort((first, second) => first.abbr.localeCompare(second.abbr))
    const newRows = document.createElement("tbody")
    filteredAbbrevs.forEach(({abbr, desc}) => {
        newRows.insertRow(-1).innerHTML = `<td class="abbrev">${abbr}</td><td class="desc">${desc}</td>`;
    })
    const oldBody = abbreviationsTable.tBodies[0]!!
    abbreviationsTable.replaceChild(newRows, oldBody)
}

filterQueryInput.oninput = () => {
    filterQueryInput.value = filterQueryInput.value.toUpperCase().replace(/[\W_]/gi, "")
    updateFilteredAbbreviations(filterQueryInput.value)
}

window.onload = () => {
    filterQueryInput.value = "";
    updateFilteredAbbreviations(filterQueryInput.value)
}