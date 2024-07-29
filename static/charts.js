var plugins = [{
    afterDraw: chart => {
        if (chart.tooltip?._active?.length) {
            let x = chart.tooltip._active[0].element.x;
            let yAxis = chart.scales.y;
            let ctx = chart.ctx;
            ctx.save();
            ctx.beginPath();
            ctx.moveTo(x, yAxis.top);
            ctx.lineTo(x, yAxis.bottom);
            ctx.lineWidth = 1;
            ctx.strokeStyle = '#ff0000';
            ctx.stroke();
            ctx.restore();
        }
    }
}];


var options = {
    maintainAspectRatio: false,

    scales: {
        x: {
            type: 'time',
            grid: {
                display: false
            }
        },

        y: {
            stacked: true,
            grid: {
                display: true,
                color: "rgba(255,99,132,0.2)"
            }
        },
    },

    interaction: {
        intersect: false,
        mode: 'index',
    },

    datasets: {
        line: {
            pointRadius: 0 // disable for all `'line'` datasets
        }
    },
}

var charts = [
    {element_id: "CarbonDioxideChart", label:"CO2 ppm", y_axis:"carbon_dioxide"},
    {element_id: "TemperatureChart", label:"Temperature", y_axis:"temperature"},
    {element_id: "HumidityChart", label:"Humidity", y_axis:"humidity"},
    {element_id: "VocChart", label:"VOC Index", y_axis:"voc_index"},
    {element_id: "NOxChart", label:"NOx Index", y_axis:"nox_index"},
    {element_id: "PM2_5Chart", label:"PM 2.5", y_axis:"pm2_5"},
    {element_id: "PM1_0Chart", label:"PM 1.0", y_axis:"pm1_0"},
    {element_id: "PM10Chart", label:"PM 10", y_axis:"pm10"}
];

var last_id = -1;

async function fetchJsonData(url) {
    try {
        const response = await fetch(url); // Send GET request to the URL

        if (!response.ok) { // Check if the response is OK (status code 200-299)
            throw new Error(`HTTP error! status: ${response.status}`);
        }

        const data = await response.json(); // Parse the response as JSON
        return data; // Return the JSON data
    } catch (error) {
        console.error('Error fetching JSON data:', error);
        return null; 
    }

}


function RenderChart(data, element_id, label, y_axis) {
    return new Chart(
        document.getElementById(element_id),
        {
            type: 'line',
            options: options,
            plugins: plugins,
            data: {
                datasets: [{
                    label: label,
                    data: data,
                    parsing: {xAxisKey: 'created_on', yAxisKey: y_axis},
                    fill: false,
                    borderColor: 'rgb(75, 192, 192)',
                    tension: 0.1
                }]
            }
        }
    );
}

function UpdateChart(newData, chart) {
    chart.data.datasets.forEach((dataset) => {
        dataset.data = dataset.data.concat(newData).slice(-500);
    });
    chart.update();
}


// Call the getJsonData function and use the returned data in multiple functions
async function render_charts() {
    url = `api${window.location.pathname}/sensor_readings/from_seconds/7200`
    const jsonData = await fetchJsonData(url);

    if (!jsonData) {
        return null;
    }

    last_id = jsonData[jsonData.length-1].id;


    charts.forEach(item => {
        item.chart = RenderChart(jsonData, item.element_id, item.label, item.y_axis);
    });
}


async function update_charts() {
    url = `api${window.location.pathname}/sensor_readings/from_id/${last_id}`
    const jsonData = await fetchJsonData(url);

    if (!jsonData || !jsonData.length) {
        return null;
    }

    last_id = jsonData[jsonData.length-1].id;

    charts.forEach(item => UpdateChart(
        jsonData, item.chart 
    ));

}

render_charts();
setInterval(update_charts, 60000);

