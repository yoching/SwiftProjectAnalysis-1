import * as d3 from "https://cdn.skypack.dev/d3@7";

import * as gridjs from "https://unpkg.com/gridjs/dist/gridjs.umd.js";

console.log("Hello world!!");

d3.json("file_stats.json").then(showData);

function showData(data) {
    console.log(data)

    new window.gridjs.Grid({
        columns: ["Name", "Length"],
        data: data
    }).render(document.getElementById("wrapper"));
}
