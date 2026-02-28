import {describe, expect, test} from "bun:test"
import {type Entry, matchFilterAgainstQuery} from "./abbreviation-filter.ts"

describe("Filter query", () => {


    test("passes anything when empty", async () => {
        const filterQuery = ""
        const entry: Entry = {abbr: "ANNA", desc: "ANNA"};
        const filterResult = matchFilterAgainstQuery(filterQuery, entry)
        expect(filterResult).not.toBeNull()
    })

    test("doesn't match non-matching entry", async () => {
        const filterQuery = "AAA"
        const entry: Entry = {abbr: "BBB", desc: "ANNA"};
        const filterResult = matchFilterAgainstQuery(filterQuery, entry)
        expect(filterResult).toBeNull()
    })

    test("doesn't distinguish case", async () => {
        const filterQuery = "AAA"
        const mixedCaseEntry = {abbr: "aAa", desc: "description"};
        const filterResult = matchFilterAgainstQuery(filterQuery, mixedCaseEntry)
        expect(filterResult).not.toBeNull()
    })

    test("ignores leading/trailing whitespace", async () => {
        const filterQuery = "  AAA "
        const entry = {abbr: "   AAA   ", desc: "description"};
        const filterResult = matchFilterAgainstQuery(filterQuery, entry)
        expect(filterResult).not.toBeNull()
    })

    test("ignores punctuation", async () => {
        const filterQuery = "A-B"
        const entry = {abbr: "A,B", desc: "description"};
        const filterResult = matchFilterAgainstQuery(filterQuery, entry)
        expect(filterResult).not.toBeNull()
    })
    test("ignores connectives like ' and '/' or '", async () => {
        const filterQuery = "AB"
        const entry = {abbr: "A or B", desc: "description"};
        const filterResult = matchFilterAgainstQuery(filterQuery, entry)
        expect(filterResult).not.toBeNull()
    })
});



