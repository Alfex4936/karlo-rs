<?php
$ffi = FFI::cdef("
    int generate_image_ffi(const char *prompt, const char *output_prefix, const char *api_key, int batch_size);
    int generate_variations_ffi(const char *input_path, const char *output_prefix, const char *api_key, int batch_size);
", __DIR__ . "/libkarlo_php.so");

$prompt = "A computer, in new york, with a dog, rainy, steampunk";
$output_prefix = "imgs/output";
$input_path = "imgs/input.png";
$api_key = "689f8a838197c912e9255e8ef26d3234";
$batch_size = 2; // Use 0 for None

$result = $ffi->generate_image_ffi($prompt, $output_prefix, $api_key, $batch_size);

if ($result == 0) {
    echo "Image generation succeeded.\n";
} else {
    echo "Image generation failed.\n";
}

$input_path = "imgs/output_1.png";
$output_prefix = "imgs/output_variation";
$result = $ffi->generate_variations_ffi($input_path, $output_prefix, $api_key, $batch_size);

if ($result == 0) {
    echo "Variations generation succeeded.\n";
} else {
    echo "Variations generation failed.\n";
}
?>
