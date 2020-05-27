const checkSize = () => {
    $.ajax({
            url: `/api/queue_size`,
            success: response => {
                console.log(response);
                $('#queue_size').text(response['size']);
                setTimeout(checkSize, 1000);
            },
            dataType: 'json'
    });
};

$(document).ready(function() {
    checkSize();
});