# Karlo-rs PHP wrapper

Synchronous karlo-rs

## Use example/libkarlo_php.so

or you can compile by yourself.

Windows user will be `.dll`


```bash
cargo build --release
```

and change php content.

```php
<?php
$ffi = FFI::cdef("
    int generate_image_ffi(const char *prompt, const char *output_prefix, const char *api_key, int batch_size);
    int generate_variations_ffi(const char *input_path, const char *output_prefix, const char *api_key, int batch_size);
", __DIR__ . "/libkarlo_php.so HERE");
?>
```

## Installation of php

1. We have to install php FFI mode.

or you can download from the website: [@Download](https://www.php.net/downloads.php)
```bash
wget https://www.php.net/distributions/php-8.2.4.tar.gz
```

2. Extract the archive:

```bash
tar -xvf php-8.2.4.tar.gz
```

3. Change to the extracted directory:

```bash
cd php-8.2.4
```

4. Configure PHP with the --with-ffi switch:

```bash
./configure --with-ffi
```

5. Compile PHP:

```bash
make
```

6. Install PHP:
   
```bash
sudo make install
```