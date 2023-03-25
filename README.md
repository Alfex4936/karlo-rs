# [karlo-rs](https://crates.io/crates/karlo-rs)

## Rust 버전 설치
```bash
cargo install karlo-rs
```

## Python 버전 설치

> Python 버전은 pyo3_src 폴더를 확인하세요.

```bash
pip install karlo-py
```
  
[Karlo](https://developers.kakao.com/docs/latest/ko/karlo/common)는 카카오에서 만든 사용자가 입력한 문장과 이미지를 기반으로 새로운 이미지를 만드는 기능입니다.

생성형 인공지능(AI) Karlo는 1억 8천만 장 규모의 이미지-텍스트 학습을 통해 사용자가 묘사한 내용을 이해하고,

픽셀 단위로 완전히 새로운 이미지를 생성합니다.

또한 사용자가 원하는 콘셉트에 맞춰 창작 활동을 할 수 있도록 사물, 배경, 조명, 구도, 다양한 화풍을 지원합니다.

B^EDIT에서 Karlo를 사용한 이미지 생성 기능을 간편하게 체험해 볼 수 있습니다.

[온라인 B^EDIT 체험하기](https://developers.kakao.com/docs/latest/ko/karlo/common#:~:text=%EA%B8%B0%EB%8A%A5%20%EC%86%8C%EA%B0%9C,%EB%B8%8C%EB%9D%BC%EC%9A%B0%EC%A0%80%EC%97%90%EC%84%9C%EB%A7%8C%20%EC%82%AC%EC%9A%A9%20%EA%B0%80%EB%8A%A5)

# 사용 방법

## 프롬프트

프롬프트(Prompt)는 Karlo API를 통해 이미지를 생성하는 데 필요한 입력 정보입니다.

프롬프트는 생성할 이미지를 묘사하는 제시어 또는 문장으로 구성되고, 기능별로 원본 이미지 등 추가 정보를 포함할 수 있습니다.

활용 가이드에서 프롬프트를 효과적으로 구성하는 방법과 다양한 예제를 확인할 수 있습니다.

> 지원 언어: 프롬프트의 제시어는 영문만 지원합니다. Karlo는 단순한 단어의 나열부터 장문의 묘사까지 구체적으로 이해할 수 있으므로, 영어가 유창하지 않아도 누구나 쉽게 사용해볼 수 있습니다.

# 이미지 생성하기

입력된 텍스트에 따라 이미지를 생성합니다.

생성할 이미지를 묘사하는 제시어를 구성해 전달하면, Karlo가 프롬프트의 제시어를 바탕으로 새로운 이미지를 생성합니다.

제시어로 계절과 같은 시기적 특징을 반영하도록 하거나, 특정 작가의 스타일을 사용하도록 지정할 수도 있습니다.

# 사용 예시

```rust
use dotenv::dotenv;
use karlo_rs;
use std::env;
use tokio;

#[tokio::main]
async fn main() {
    dotenv().ok();
    let api_key = env::var("API_KEY").expect("API_KEY not set in .env file");

    // Generate images based on prompt text
    let text = "A newyork home, steampunk, snowy";
    let output_name = "sample_img/output"; // will be png
    let batch_size = Some(2); // or Some(value) where value is between 1 and 8

    match karlo_rs::generate_image(text, output_name, &api_key, batch_size).await {
        Ok(_) => println!("Image saved to {}", output_name),
        Err(e) => eprintln!("Error: {}", e),
    }

    // Generate images based on input image.
    let input_path = "sample_img/output_1.png";
    let output_name = "sample_img/output_variation"; // will be png
    let batch_size = None; // or Some(value) where value is between 1 and 8

    match karlo_rs::generate_variations(input_path, output_name, &api_key, batch_size).await {
        Ok(_) => println!("Variation image saved to {}", output_name),
        Err(e) => eprintln!("Error: {}", e),
    }
}
```

## 위 실행 결과

### Prompt: A newyork home, steampunk, snowy

![output_1](https://user-images.githubusercontent.com/2356749/227417438-53289353-239a-4808-a132-b6a3b3a956c1.png)
![output_2](https://user-images.githubusercontent.com/2356749/227417446-65441ff0-ccb5-497d-a090-89c30b6f6f30.png)

### Variation
![output_variation_1](https://user-images.githubusercontent.com/2356749/227417452-a17ed016-46ae-46dc-843d-235b50bd56e5.png)
