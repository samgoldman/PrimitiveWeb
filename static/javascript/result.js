let request_status = "unknown";
const MAX_HEIGHT = $(window).innerHeight() - 100;
const MAX_WIDTH = $(window).innerWidth() - 100;

const adjustSizes = () => {
    const original_height = $("#svg_object").height();
    const original_width  = $("#svg_object").width();

    if (0 === original_height) {
        setTimeout(adjustSizes, 50);
    } else {
        console.log("adjusting");
        const height_ratio = MAX_HEIGHT / original_height;
        const width_ratio = MAX_WIDTH / original_width;

        const chosen_ratio = Math.min(height_ratio, width_ratio);
        document.querySelector('#svg_object').setAttribute('height', original_height * chosen_ratio);
        document.querySelector('#svg_object').setAttribute('width', original_width * chosen_ratio);
    }
}

const loadSvg = () => {
    $("#svg_object").attr('src', `/api/get_result/${REQUEST_ID}`);
    adjustSizes();

    $('#delete_notice').css('display', 'block');
    $('#download_link').css('display', 'block');
};

const checkStatus = () => {
    $.ajax({
            url: `/api/check_status/${REQUEST_ID}`,
            success: response => {
                request_status = response['status']
                $('#status').text(request_status);

                // If pending, keep checking
                if ('pending' === request_status) {
                    setTimeout(checkStatus, 200);
                } else if ('done' === request_status) {
                    loadSvg();
                }
            },
            dataType: 'json'
    });
};

$(document).ready(function() {
    checkStatus();
});