// wait for the DOM to be loaded
$(document).ready(function() {
    $('#submission_form').ajaxForm({url : '/api/submit',
       dataType : 'json',
       type: 'POST',
       success : function (response) {
            if (response['status'] === 'ok') {
                window.location.href = "/view/result/" + response['request_id'];
            }
       }
    });
});