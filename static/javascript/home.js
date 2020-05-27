// wait for the DOM to be loaded
$(document).ready(function() {
    bsCustomFileInput.init();

    $("#submission_form").submit(function(e) {
        e.preventDefault();
        const formData = new FormData(this);

        $.post({
            url: '/api/submit',
            data: formData,
            contentType: false,
            processData: false,
            success: function(response) {
                if (response['status'] === 'ok') {
                    window.location.href = "/view/result/" + response['request_id'];
                } else {
                    $('#failure_message').text('Error: ' + response['message']);
                    $('#failure_message').css('display', 'block');
                }
        }});
    });
});