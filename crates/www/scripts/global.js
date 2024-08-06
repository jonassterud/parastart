"use strict"

/**
 * Fetch takeoffs.
 * 
 * @param {Number} [page] - Optional page offset (`=>1`).
 * @param {Number} [limit] - Optional number of takeoffs to fetch.
 * @param {String} [region] - Optional name of region.
 * @param {Array<String>} [fields] Optional list of columns to fetch
 * @returns {Array<Object>} A list of takeoffs as objects.
 */
async function fetch_takeoffs(page, limit, region, fields) {
    try {
        const url = new URL(window.location.origin);
        url.pathname = "/api/v0/takeoffs";
        
        if (page !== undefined) url.searchParams.append("page", page);
        if (limit !== undefined) url.searchParams.append("limit", limit);
        if (region !== undefined) url.searchParams.append("region", region);
        fields?.forEach((field) => url.searchParams.append("fields", field));

        const response = await fetch(url);
        const json = await response.json();

        return json;
    } catch (error) {
        throw error;
    }
}

/**
 * Fetch takeoff.
 * 
 * @param {Number} id - Takeoff id.
 * @returns {Object} A takeoff object.
 */
async function fetch_takeoff(id) {
    try {
        let path = `/api/v0/takeoffs?id=${id}`;

        const response = await fetch(path);
        const json = await response.json();
        const takeoff = json.at(0);

        if (takeoff === undefined) {
            throw new Error("takeoff is undefined");
        }

        return takeoff;
    } catch (error) {
        throw error;
    }
}
