document.addEventListener("DOMContentLoaded", function () {
    const infoIcon = document.querySelector(".info-icon");
    const tooltipContent = document.querySelector(".tooltip-content");

    infoIcon.addEventListener("mouseover", () => {
        tooltipContent.style.display = "block";
    });

    infoIcon.addEventListener("mouseout", () => {
        tooltipContent.style.display = "none";
    });
});
