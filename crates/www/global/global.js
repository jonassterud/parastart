"use strict"

/**
 * Fetch takeoffs.
 * 
 * @param {Number} page - Page offset (`=>1`).
 * @param {Number} limit - Number of takeoffs to fetch.
 * @param {String} [region] - Optional name of region.
 * @returns {Array<Object>} A list of takeoffs as objects.
 */
async function fetch_takeoffs(page=1, limit=10, region) {
    try {
        let path = `/api/v0/takeoffs?page=${page}&limit=${limit}`;
        if (region !== undefined) path += `&region=${region}`;

        const response = await fetch(path);
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
