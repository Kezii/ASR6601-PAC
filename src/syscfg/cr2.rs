#[doc = "Register `CR2` reader"]
pub type R = crate::R<Cr2Spec>;
#[doc = "Register `CR2` writer"]
pub type W = crate::W<Cr2Spec>;
#[doc = "Field `SSP_AFEC_DMA_CLR_SEL` reader - ssp for afec dma clr signal selection"]
pub type SspAfecDmaClrSelR = crate::BitReader;
#[doc = "Field `SSP_AFEC_DMA_CLR_SEL` writer - ssp for afec dma clr signal selection"]
pub type SspAfecDmaClrSelW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SSP2_DMA_CLR_SEL` reader - ssp2 dma clr signal selection"]
pub type Ssp2DmaClrSelR = crate::BitReader;
#[doc = "Field `SSP2_DMA_CLR_SEL` writer - ssp2 dma clr signal selection"]
pub type Ssp2DmaClrSelW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SSP1_DMA_CLR_SEL` reader - ssp1 dma clr signal selection"]
pub type Ssp1DmaClrSelR = crate::BitReader;
#[doc = "Field `SSP1_DMA_CLR_SEL` writer - ssp1 dma clr signal selection"]
pub type Ssp1DmaClrSelW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SSP0_DMA_CLR_SEL` reader - ssp0 dma clr signal selection"]
pub type Ssp0DmaClrSelR = crate::BitReader;
#[doc = "Field `SSP0_DMA_CLR_SEL` writer - ssp0 dma clr signal selection"]
pub type Ssp0DmaClrSelW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UART3_DMA_CLR_SEL` reader - uart3 dma clr signal selection"]
pub type Uart3DmaClrSelR = crate::BitReader;
#[doc = "Field `UART3_DMA_CLR_SEL` writer - uart3 dma clr signal selection"]
pub type Uart3DmaClrSelW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UART2_DMA_CLR_SEL` reader - uart2 dma clr signal selection"]
pub type Uart2DmaClrSelR = crate::BitReader;
#[doc = "Field `UART2_DMA_CLR_SEL` writer - uart2 dma clr signal selection"]
pub type Uart2DmaClrSelW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UART1_DMA_CLR_SEL` reader - uart1 dma clr signal selection"]
pub type Uart1DmaClrSelR = crate::BitReader;
#[doc = "Field `UART1_DMA_CLR_SEL` writer - uart1 dma clr signal selection"]
pub type Uart1DmaClrSelW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UART0_DMA_CLR_SEL` reader - uart0 dma clr signal selection"]
pub type Uart0DmaClrSelR = crate::BitReader;
#[doc = "Field `UART0_DMA_CLR_SEL` writer - uart0 dma clr signal selection"]
pub type Uart0DmaClrSelW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SYSCFG_DBG_SLEEP` reader - allow debug connection in deepsleep mode"]
pub type SyscfgDbgSleepR = crate::BitReader;
#[doc = "Field `SYSCFG_DBG_SLEEP` writer - allow debug connection in deepsleep mode"]
pub type SyscfgDbgSleepW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CPU_STCALIB_SKEW` reader - cpu systick skew configuration"]
pub type CpuStcalibSkewR = crate::BitReader;
#[doc = "Field `CPU_STCALIB_SKEW` writer - cpu systick skew configuration"]
pub type CpuStcalibSkewW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `QSPI_REMAP_ENABLE` reader - qspi remap function enable"]
pub type QspiRemapEnableR = crate::BitReader;
#[doc = "Field `QSPI_REMAP_ENABLE` writer - qspi remap function enable"]
pub type QspiRemapEnableW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `QSPI_MEM_ENCRYPT_EN` reader - qspi memory encryption enable"]
pub type QspiMemEncryptEnR = crate::BitReader;
#[doc = "Field `QSPI_MEM_ENCRYPT_EN` writer - qspi memory encryption enable"]
pub type QspiMemEncryptEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SYSCFG_HALTED_BASICTIM1_EN` reader - halt basictim1 counter when core is halted"]
pub type SyscfgHaltedBasictim1EnR = crate::BitReader;
#[doc = "Field `SYSCFG_HALTED_BASICTIM1_EN` writer - halt basictim1 counter when core is halted"]
pub type SyscfgHaltedBasictim1EnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SYSCFG_HALTED_BASICTIM0_EN` reader - halt basictim0 counter when core is halted"]
pub type SyscfgHaltedBasictim0EnR = crate::BitReader;
#[doc = "Field `SYSCFG_HALTED_BASICTIM0_EN` writer - halt basictim0 counter when core is halted"]
pub type SyscfgHaltedBasictim0EnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SYSCFG_HALTED_GPTIM3_EN` reader - halt gptim3 counter when core is halted"]
pub type SyscfgHaltedGptim3EnR = crate::BitReader;
#[doc = "Field `SYSCFG_HALTED_GPTIM3_EN` writer - halt gptim3 counter when core is halted"]
pub type SyscfgHaltedGptim3EnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SYSCFG_HALTED_GPTIM2_EN` reader - halt gptim2 counter when core is halted"]
pub type SyscfgHaltedGptim2EnR = crate::BitReader;
#[doc = "Field `SYSCFG_HALTED_GPTIM2_EN` writer - halt gptim2 counter when core is halted"]
pub type SyscfgHaltedGptim2EnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SYSCFG_HALTED_GPTIM1_EN` reader - halt gptim1 counter when core is halted"]
pub type SyscfgHaltedGptim1EnR = crate::BitReader;
#[doc = "Field `SYSCFG_HALTED_GPTIM1_EN` writer - halt gptim1 counter when core is halted"]
pub type SyscfgHaltedGptim1EnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SYSCFG_HALTED_GPTIM0_EN` reader - halt gptim0 counter when core is halted"]
pub type SyscfgHaltedGptim0EnR = crate::BitReader;
#[doc = "Field `SYSCFG_HALTED_GPTIM0_EN` writer - halt gptim0 counter when core is halted"]
pub type SyscfgHaltedGptim0EnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SYSCFG_HALTED_WWDG_EN` reader - halt window watchdog counter when core is halted"]
pub type SyscfgHaltedWwdgEnR = crate::BitReader;
#[doc = "Field `SYSCFG_HALTED_WWDG_EN` writer - halt window watchdog counter when core is halted"]
pub type SyscfgHaltedWwdgEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SYSCFG_HALTED_IWDG_EN` reader - halt independent watchdog counter when core is halted"]
pub type SyscfgHaltedIwdgEnR = crate::BitReader;
#[doc = "Field `SYSCFG_HALTED_IWDG_EN` writer - halt independent watchdog counter when core is halted"]
pub type SyscfgHaltedIwdgEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SYSCFG_HALTED_LPTIM0_EN` reader - halt lptim0 counter when core is halted"]
pub type SyscfgHaltedLptim0EnR = crate::BitReader;
#[doc = "Field `SYSCFG_HALTED_LPTIM0_EN` writer - halt lptim0 counter when core is halted"]
pub type SyscfgHaltedLptim0EnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SYSCFG_HALTED_LPTIM1_EN` reader - halt lptim1 counter when core is halted"]
pub type SyscfgHaltedLptim1EnR = crate::BitReader;
#[doc = "Field `SYSCFG_HALTED_LPTIM1_EN` writer - halt lptim1 counter when core is halted"]
pub type SyscfgHaltedLptim1EnW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - ssp for afec dma clr signal selection"]
    #[inline(always)]
    pub fn ssp_afec_dma_clr_sel(&self) -> SspAfecDmaClrSelR {
        SspAfecDmaClrSelR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - ssp2 dma clr signal selection"]
    #[inline(always)]
    pub fn ssp2_dma_clr_sel(&self) -> Ssp2DmaClrSelR {
        Ssp2DmaClrSelR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - ssp1 dma clr signal selection"]
    #[inline(always)]
    pub fn ssp1_dma_clr_sel(&self) -> Ssp1DmaClrSelR {
        Ssp1DmaClrSelR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - ssp0 dma clr signal selection"]
    #[inline(always)]
    pub fn ssp0_dma_clr_sel(&self) -> Ssp0DmaClrSelR {
        Ssp0DmaClrSelR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - uart3 dma clr signal selection"]
    #[inline(always)]
    pub fn uart3_dma_clr_sel(&self) -> Uart3DmaClrSelR {
        Uart3DmaClrSelR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - uart2 dma clr signal selection"]
    #[inline(always)]
    pub fn uart2_dma_clr_sel(&self) -> Uart2DmaClrSelR {
        Uart2DmaClrSelR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - uart1 dma clr signal selection"]
    #[inline(always)]
    pub fn uart1_dma_clr_sel(&self) -> Uart1DmaClrSelR {
        Uart1DmaClrSelR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - uart0 dma clr signal selection"]
    #[inline(always)]
    pub fn uart0_dma_clr_sel(&self) -> Uart0DmaClrSelR {
        Uart0DmaClrSelR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 10 - allow debug connection in deepsleep mode"]
    #[inline(always)]
    pub fn syscfg_dbg_sleep(&self) -> SyscfgDbgSleepR {
        SyscfgDbgSleepR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - cpu systick skew configuration"]
    #[inline(always)]
    pub fn cpu_stcalib_skew(&self) -> CpuStcalibSkewR {
        CpuStcalibSkewR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 17 - qspi remap function enable"]
    #[inline(always)]
    pub fn qspi_remap_enable(&self) -> QspiRemapEnableR {
        QspiRemapEnableR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - qspi memory encryption enable"]
    #[inline(always)]
    pub fn qspi_mem_encrypt_en(&self) -> QspiMemEncryptEnR {
        QspiMemEncryptEnR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - halt basictim1 counter when core is halted"]
    #[inline(always)]
    pub fn syscfg_halted_basictim1_en(&self) -> SyscfgHaltedBasictim1EnR {
        SyscfgHaltedBasictim1EnR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - halt basictim0 counter when core is halted"]
    #[inline(always)]
    pub fn syscfg_halted_basictim0_en(&self) -> SyscfgHaltedBasictim0EnR {
        SyscfgHaltedBasictim0EnR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - halt gptim3 counter when core is halted"]
    #[inline(always)]
    pub fn syscfg_halted_gptim3_en(&self) -> SyscfgHaltedGptim3EnR {
        SyscfgHaltedGptim3EnR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - halt gptim2 counter when core is halted"]
    #[inline(always)]
    pub fn syscfg_halted_gptim2_en(&self) -> SyscfgHaltedGptim2EnR {
        SyscfgHaltedGptim2EnR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - halt gptim1 counter when core is halted"]
    #[inline(always)]
    pub fn syscfg_halted_gptim1_en(&self) -> SyscfgHaltedGptim1EnR {
        SyscfgHaltedGptim1EnR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - halt gptim0 counter when core is halted"]
    #[inline(always)]
    pub fn syscfg_halted_gptim0_en(&self) -> SyscfgHaltedGptim0EnR {
        SyscfgHaltedGptim0EnR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - halt window watchdog counter when core is halted"]
    #[inline(always)]
    pub fn syscfg_halted_wwdg_en(&self) -> SyscfgHaltedWwdgEnR {
        SyscfgHaltedWwdgEnR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - halt independent watchdog counter when core is halted"]
    #[inline(always)]
    pub fn syscfg_halted_iwdg_en(&self) -> SyscfgHaltedIwdgEnR {
        SyscfgHaltedIwdgEnR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - halt lptim0 counter when core is halted"]
    #[inline(always)]
    pub fn syscfg_halted_lptim0_en(&self) -> SyscfgHaltedLptim0EnR {
        SyscfgHaltedLptim0EnR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 30 - halt lptim1 counter when core is halted"]
    #[inline(always)]
    pub fn syscfg_halted_lptim1_en(&self) -> SyscfgHaltedLptim1EnR {
        SyscfgHaltedLptim1EnR::new(((self.bits >> 30) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - ssp for afec dma clr signal selection"]
    #[inline(always)]
    pub fn ssp_afec_dma_clr_sel(&mut self) -> SspAfecDmaClrSelW<'_, Cr2Spec> {
        SspAfecDmaClrSelW::new(self, 0)
    }
    #[doc = "Bit 1 - ssp2 dma clr signal selection"]
    #[inline(always)]
    pub fn ssp2_dma_clr_sel(&mut self) -> Ssp2DmaClrSelW<'_, Cr2Spec> {
        Ssp2DmaClrSelW::new(self, 1)
    }
    #[doc = "Bit 2 - ssp1 dma clr signal selection"]
    #[inline(always)]
    pub fn ssp1_dma_clr_sel(&mut self) -> Ssp1DmaClrSelW<'_, Cr2Spec> {
        Ssp1DmaClrSelW::new(self, 2)
    }
    #[doc = "Bit 3 - ssp0 dma clr signal selection"]
    #[inline(always)]
    pub fn ssp0_dma_clr_sel(&mut self) -> Ssp0DmaClrSelW<'_, Cr2Spec> {
        Ssp0DmaClrSelW::new(self, 3)
    }
    #[doc = "Bit 4 - uart3 dma clr signal selection"]
    #[inline(always)]
    pub fn uart3_dma_clr_sel(&mut self) -> Uart3DmaClrSelW<'_, Cr2Spec> {
        Uart3DmaClrSelW::new(self, 4)
    }
    #[doc = "Bit 5 - uart2 dma clr signal selection"]
    #[inline(always)]
    pub fn uart2_dma_clr_sel(&mut self) -> Uart2DmaClrSelW<'_, Cr2Spec> {
        Uart2DmaClrSelW::new(self, 5)
    }
    #[doc = "Bit 6 - uart1 dma clr signal selection"]
    #[inline(always)]
    pub fn uart1_dma_clr_sel(&mut self) -> Uart1DmaClrSelW<'_, Cr2Spec> {
        Uart1DmaClrSelW::new(self, 6)
    }
    #[doc = "Bit 7 - uart0 dma clr signal selection"]
    #[inline(always)]
    pub fn uart0_dma_clr_sel(&mut self) -> Uart0DmaClrSelW<'_, Cr2Spec> {
        Uart0DmaClrSelW::new(self, 7)
    }
    #[doc = "Bit 10 - allow debug connection in deepsleep mode"]
    #[inline(always)]
    pub fn syscfg_dbg_sleep(&mut self) -> SyscfgDbgSleepW<'_, Cr2Spec> {
        SyscfgDbgSleepW::new(self, 10)
    }
    #[doc = "Bit 11 - cpu systick skew configuration"]
    #[inline(always)]
    pub fn cpu_stcalib_skew(&mut self) -> CpuStcalibSkewW<'_, Cr2Spec> {
        CpuStcalibSkewW::new(self, 11)
    }
    #[doc = "Bit 17 - qspi remap function enable"]
    #[inline(always)]
    pub fn qspi_remap_enable(&mut self) -> QspiRemapEnableW<'_, Cr2Spec> {
        QspiRemapEnableW::new(self, 17)
    }
    #[doc = "Bit 18 - qspi memory encryption enable"]
    #[inline(always)]
    pub fn qspi_mem_encrypt_en(&mut self) -> QspiMemEncryptEnW<'_, Cr2Spec> {
        QspiMemEncryptEnW::new(self, 18)
    }
    #[doc = "Bit 19 - halt basictim1 counter when core is halted"]
    #[inline(always)]
    pub fn syscfg_halted_basictim1_en(&mut self) -> SyscfgHaltedBasictim1EnW<'_, Cr2Spec> {
        SyscfgHaltedBasictim1EnW::new(self, 19)
    }
    #[doc = "Bit 20 - halt basictim0 counter when core is halted"]
    #[inline(always)]
    pub fn syscfg_halted_basictim0_en(&mut self) -> SyscfgHaltedBasictim0EnW<'_, Cr2Spec> {
        SyscfgHaltedBasictim0EnW::new(self, 20)
    }
    #[doc = "Bit 21 - halt gptim3 counter when core is halted"]
    #[inline(always)]
    pub fn syscfg_halted_gptim3_en(&mut self) -> SyscfgHaltedGptim3EnW<'_, Cr2Spec> {
        SyscfgHaltedGptim3EnW::new(self, 21)
    }
    #[doc = "Bit 22 - halt gptim2 counter when core is halted"]
    #[inline(always)]
    pub fn syscfg_halted_gptim2_en(&mut self) -> SyscfgHaltedGptim2EnW<'_, Cr2Spec> {
        SyscfgHaltedGptim2EnW::new(self, 22)
    }
    #[doc = "Bit 23 - halt gptim1 counter when core is halted"]
    #[inline(always)]
    pub fn syscfg_halted_gptim1_en(&mut self) -> SyscfgHaltedGptim1EnW<'_, Cr2Spec> {
        SyscfgHaltedGptim1EnW::new(self, 23)
    }
    #[doc = "Bit 24 - halt gptim0 counter when core is halted"]
    #[inline(always)]
    pub fn syscfg_halted_gptim0_en(&mut self) -> SyscfgHaltedGptim0EnW<'_, Cr2Spec> {
        SyscfgHaltedGptim0EnW::new(self, 24)
    }
    #[doc = "Bit 25 - halt window watchdog counter when core is halted"]
    #[inline(always)]
    pub fn syscfg_halted_wwdg_en(&mut self) -> SyscfgHaltedWwdgEnW<'_, Cr2Spec> {
        SyscfgHaltedWwdgEnW::new(self, 25)
    }
    #[doc = "Bit 26 - halt independent watchdog counter when core is halted"]
    #[inline(always)]
    pub fn syscfg_halted_iwdg_en(&mut self) -> SyscfgHaltedIwdgEnW<'_, Cr2Spec> {
        SyscfgHaltedIwdgEnW::new(self, 26)
    }
    #[doc = "Bit 27 - halt lptim0 counter when core is halted"]
    #[inline(always)]
    pub fn syscfg_halted_lptim0_en(&mut self) -> SyscfgHaltedLptim0EnW<'_, Cr2Spec> {
        SyscfgHaltedLptim0EnW::new(self, 27)
    }
    #[doc = "Bit 30 - halt lptim1 counter when core is halted"]
    #[inline(always)]
    pub fn syscfg_halted_lptim1_en(&mut self) -> SyscfgHaltedLptim1EnW<'_, Cr2Spec> {
        SyscfgHaltedLptim1EnW::new(self, 30)
    }
}
#[doc = "control register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`cr2::R`](R). You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cr2Spec;
impl crate::RegisterSpec for Cr2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cr2::R`](R) reader structure"]
impl crate::Readable for Cr2Spec {}
#[doc = "`write(|w| ..)` method takes [`cr2::W`](W) writer structure"]
impl crate::Writable for Cr2Spec {
    type Safety = crate::Unsafe;
}
