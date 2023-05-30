getIncomeData();

function getIncomeData() {
    fetch("/income_data")
    .then(res => res.json())
    .then(incomeData => {
        setIncome(incomeData);
    });
}

function setIncome(data) {
    document.getElementById("salary").innerText = `${data.salary}`;
    document.getElementById("tax-rate").innerText = `${data.tax_rate * 100}`;
    document.getElementById("taxed-salary").innerText = `${Math.floor((data.salary - (data.salary * data.tax_rate)) * 100.00) / 100.00}`;
    document.getElementById("hours-worked").innerText = `${data.hours_worked}`;
    document.getElementById("monthsTotal").innerText = `${data.months_total}`;

    document.getElementById("add-hours-worked-button").addEventListener("click", () => {
        document.getElementById("update-hours-dialog").showModal();
        document.getElementById("update-hours-form").addEventListener("submit", () => {
            updateHoursWorked();
        })
    });

    document.getElementById("reset-hours-worked-button").addEventListener("click", () => {
        document.getElementById("reset-hours-dialog").showModal();
        document.getElementById("reset-hours-confirm").addEventListener("click", () => {
            resetHoursWorked();
        });
        document.getElementById("reset-hours-cancel").addEventListener("click", () => document.getElementById("reset-hours-dialog").closel())
    });
}

function updateHoursWorked() {
    /**
     * When form is submitted, send data to the server
     * Server will calculate and store the data.
     */
    let submittedHours = document.forms["update-hours-form"]["update-hours-input"].value;
    let mathedMinutes = Math.floor((document.forms["update-hours-form"]["update-minutes-input"].value / 60) * 100);
    fetch("/income_data/updated_hours", { method: "POST", body: `${submittedHours}.${mathedMinutes}` })
        .then(res => res.json())
        .then(data => {
            if (data.status == "200") {
                window.location.reload();
                return;
            }
            alert("Something went wrong...");
            window.location.reload();
        });
}

function resetHoursWorked() {
    fetch("/income_data/reset_hours", { method: "POST", body: "" })
        .then(res => res.json())
        .then(data => {
            if (data.status == "200") {
                window.location.reload();
                return;
            }
            alert("Something went wrong...");
            window.location.reload();
        });
}