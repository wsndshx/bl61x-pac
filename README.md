# BL616-PAC

嵌入式Rust的外设访问包装库，适用于BL616微控制器（BL616/BL618）。

Embedded Rust's Peripheral Access Crate for BL616 microcontrollers(BL616/BL618).

该项目是通过使用 `svd2rust` 从SVD文件自动生成的。

This project is automatic generated from SVD file using `svd2rust`.

## 注意

**该项目仍处于初期阶段，并且有许多寄存器并未得到维护。**项目所使用的SVD文件来源于Bouffalo Lab维护的仓库[bouffalolab/bl-pac (github.com)](https://github.com/bouffalolab/bl-pac)，该项目基于Bouffalo Lab提供的SVD文件与[bouffalolab/bouffalo_sdk (github.com)](https://github.com/bouffalolab/bouffalo_sdk)中的寄存器文件进行修补。

**The project is still in its early stages, and many registers have not been maintained.** The SVD file used in the project is sourced from the repository maintained by Bouffalo Lab [bouffalolab/bl-pac (github.com)](https://github.com/bouffalolab/bl-pac). This project involves patching the original SVD file based on the files available in [bouffalolab/bouffalo_sdk (github.com)](https://github.com/bouffalolab/bouffalo_sdk) and the SVD files provided by Bouffalo Lab.