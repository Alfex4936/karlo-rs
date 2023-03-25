import asyncio
import os

from karlo_py import gen_image, gen_variation


async def main():
    API_KEY = os.getenv("API_KEY")

    if API_KEY is None:
        raise ValueError("API_KEY not set in environment")

    text = "A South korea in cyberpunk style, rainy"
    output_name = "py_img/output"  # will be png
    batch_size = 2  # or an integer between 1 and 8

    try:
        await gen_image(text, output_name, API_KEY, batch_size)
        print(f"Image saved to {output_name}")
    except Exception as e:
        print(e)

    input_path = "py_img/output_1.png"
    output_name = "py_img/output_variation"  # will be png
    batch_size = None  # or an integer between 1 and 8

    try:
        await gen_variation(input_path, output_name, API_KEY, batch_size)
        print(f"Variation image saved to {output_name}")
    except Exception as e:
        print(f"Error: {e}")


if __name__ == "__main__":
    asyncio.run(main())
