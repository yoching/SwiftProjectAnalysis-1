import * as d3 from "https://cdn.skypack.dev/d3@7";

import * as gridjs from "https://unpkg.com/gridjs/dist/gridjs.umd.js";

console.log("Hello world!!");

d3
    .json("file_stats.json")
    .then((data) => {
        showTable(data);
        showPie(data);
    });

function showTable(data) {
    console.log(data)

    data.sort(function (a, b) {
        if (a.length < b.length) return 1;
        if (a.length > b.length) return -1;
        return 0;
    })

    console.log("sorted data")
    console.log(data)

    new window.gridjs.Grid(
        {
            columns: [
                {
                    "id": 'name',
                    "name": "File name"
                },
                {
                    "id": 'length',
                    "name": "File length"
                },
                {
                    "id": 'struct_count',
                    "name": "Struct"
                },
                {
                    "id": 'class_count',
                    "name": "Class"
                },
                {
                    "id": 'enum_count',
                    "name": "Enum"
                }
            ],
            sort: true,
            pagination: true,
            data: data
        }
    )
        .render(document.getElementById("wrapper"));
}

function showPie(data) {
    let bodyHeight = 200
    let bodyWidth = 400

    let all_struct_count = data
        .map(d => +d.struct_count)
        .reduce((a, b) => a + b, 0)

    let all_enum_count = data
        .map(d => +d.enum_count)
        .reduce((a, b) => a + b, 0)

    let all_class_count = data
        .map(d => +d.class_count)
        .reduce((a, b) => a + b, 0)

    let pie_source = [
        {
            "element": "struct",
            "count": all_struct_count,
        },
        {
            "element": "class",
            "count": all_class_count,
        },
        {
            "element": "enum",
            "count": all_enum_count,
        },
    ]
    console.log(pie_source);

    let pie = d3.pie()
        .value(d => d.count)

    let colorScale = d3.scaleOrdinal()
        .range(d3.schemeSet2)
        .domain(pie_source.map(d => d.element))

    let arc = d3.arc()
        .outerRadius(bodyHeight / 2)
        .innerRadius(0)

    let g =
        d3.select("#arc").select("#body").selectAll(".arc")
            .data(pie(pie_source))
            .enter()
            .append("g")

    g.append("path")
        .attr("d", arc)
        .attr("fill", d => {
            return colorScale(d.data.element)
        })

    let annotation = g
        .append('text')
        .attr("transform", function (d) {
            let centroid = arc.centroid(d);
            return "translate(" + centroid[0] + "," + (centroid[1] - 15) + ")";
        })
        .style("text-anchor", "middle")
        .style("font-size", 15)

    annotation
        .append('tspan')
        .text(d => {
            let string = d.data.element
            console.log(string);
            return string
        })
        .attr("dy", ".6em")
        .attr("x", 0)

    annotation
        .append('tspan')
        .text(d => {
            return d.data.count
        })
        .attr("dy", "1.2em")
        .attr("x", 0)

}