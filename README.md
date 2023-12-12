# BL616-PAC

嵌入式Rust的外设访问包装库，适用于BL616微控制器（BL616/BL618）。

Embedded Rust's Peripheral Access Crate for BL616 microcontrollers(BL616/BL618).

该项目是通过使用 `svd2rust` 从SVD文件自动生成的。

This project is automatic generated from SVD file using `svd2rust`.

## 注意

**该项目仍处于初期阶段，并且有许多寄存器并未得到维护。**

**The project is still in its early stages, and many registers have not been maintained.**

该项目中的SVD文件来源于Bouffalo Lab维护的仓库[bouffalolab/bl-pac (github.com)](https://github.com/bouffalolab/bl-pac)。然而，由于该仓库目前似乎缺乏维护，因此SVD文件中未包含部分外设。为此，本项目在参考[bouffalolab/bouffalo_sdk (github.com)](https://github.com/bouffalolab/bouffalo_sdk)中的代码的基础上，制作了一份修补后的SVD文件。

The SVD file used in this project is sourced from the repository maintained by Bouffalo Lab [bouffalolab/bl-pac (github.com)](https://github.com/bouffalolab/bl-pac). However, due to apparent lack of maintenance in this repository, certain peripheral information is missing from the SVD file. To address this issue, this project has created a patched SVD file based on references from the code in [bouffalolab/bouffalo_sdk (github.com)](https://github.com/bouffalolab/bouffalo_sdk).

如果以后官方仓库得到了更新并变得可用了，那么本项目将会停止更新，并建议用户使用官方维护的仓库而非第三方维护的仓库。我们始终鼓励和推荐使用官方维护的资源，以确保项目在更新性和可靠性方面得到最佳支持。

If the official repository becomes updated and accessible in the future, this project will cease further updates. It is advised that users switch to the officially maintained repository rather than relying on third-party maintained repositories. We consistently encourage and recommend the use of officially maintained resources to ensure optimal support for the project in terms of updates and reliability.