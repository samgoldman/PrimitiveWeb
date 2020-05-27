// wait for the DOM to be loaded
$(document).ready(function() {
    bsCustomFileInput.init();
    $('#submission_form').ajaxForm({url : '/api/submit',
       dataType : 'json',
       type: 'POST',
       success : response => {
            if (response['status'] === 'ok') {
                window.location.href = "/view/result/" + response['request_id'];
            }
       }
    });
});