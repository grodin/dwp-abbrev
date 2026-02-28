export type Entry = {
    abbr: string, desc: string,
}

const CONNECTIVES = [" and ", " to ", " or "]

export function matchFilterAgainstQuery(filterQuery: string, entry: Entry): Entry | null {
    if (filterQuery === "") return entry

    function processAbbreviation(input: string): string {
        return input.trim().toUpperCase().replace(/[\W_]/gi, "")
    }

    const processedQuery = processAbbreviation(filterQuery)
    const processedAbbrev = processAbbreviation(entry.abbr)
    const processedAbbrevWithoutConnectives = processAbbreviation(CONNECTIVES.reduce((acc, c) => acc.replaceAll(c, ""), entry.abbr))
    if (processedAbbrev.startsWith(processedQuery) || processedAbbrevWithoutConnectives.startsWith(processedQuery)) {
        return entry
    }
    return null
}