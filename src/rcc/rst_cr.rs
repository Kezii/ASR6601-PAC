#[doc = "Register `RST_CR` reader"]
pub type R = crate::R<RstCrSpec>;
#[doc = "Register `RST_CR` writer"]
pub type W = crate::W<RstCrSpec>;
#[doc = "Field `SEC_RESET_REQ_EN` reader - Sec reset req en"]
pub type SecResetReqEnR = crate::BitReader;
#[doc = "Field `SEC_RESET_REQ_EN` writer - Sec reset req en"]
pub type SecResetReqEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CPU_RESET_REQ_EN` reader - Cpu reset req en"]
pub type CpuResetReqEnR = crate::BitReader;
#[doc = "Field `CPU_RESET_REQ_EN` writer - Cpu reset req en"]
pub type CpuResetReqEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EFC_RESET_REQ_EN` reader - Efc reset req en"]
pub type EfcResetReqEnR = crate::BitReader;
#[doc = "Field `EFC_RESET_REQ_EN` writer - Efc reset req en"]
pub type EfcResetReqEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WDG_RESET_REQ_EN` reader - Wdg reset req en"]
pub type WdgResetReqEnR = crate::BitReader;
#[doc = "Field `WDG_RESET_REQ_EN` writer - Wdg reset req en"]
pub type WdgResetReqEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IWDG_RESET_REQ_EN` reader - Iwdg reset req en"]
pub type IwdgResetReqEnR = crate::BitReader;
#[doc = "Field `IWDG_RESET_REQ_EN` writer - Iwdg reset req en"]
pub type IwdgResetReqEnW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 1 - Sec reset req en"]
    #[inline(always)]
    pub fn sec_reset_req_en(&self) -> SecResetReqEnR {
        SecResetReqEnR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Cpu reset req en"]
    #[inline(always)]
    pub fn cpu_reset_req_en(&self) -> CpuResetReqEnR {
        CpuResetReqEnR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Efc reset req en"]
    #[inline(always)]
    pub fn efc_reset_req_en(&self) -> EfcResetReqEnR {
        EfcResetReqEnR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Wdg reset req en"]
    #[inline(always)]
    pub fn wdg_reset_req_en(&self) -> WdgResetReqEnR {
        WdgResetReqEnR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Iwdg reset req en"]
    #[inline(always)]
    pub fn iwdg_reset_req_en(&self) -> IwdgResetReqEnR {
        IwdgResetReqEnR::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - Sec reset req en"]
    #[inline(always)]
    pub fn sec_reset_req_en(&mut self) -> SecResetReqEnW<'_, RstCrSpec> {
        SecResetReqEnW::new(self, 1)
    }
    #[doc = "Bit 2 - Cpu reset req en"]
    #[inline(always)]
    pub fn cpu_reset_req_en(&mut self) -> CpuResetReqEnW<'_, RstCrSpec> {
        CpuResetReqEnW::new(self, 2)
    }
    #[doc = "Bit 3 - Efc reset req en"]
    #[inline(always)]
    pub fn efc_reset_req_en(&mut self) -> EfcResetReqEnW<'_, RstCrSpec> {
        EfcResetReqEnW::new(self, 3)
    }
    #[doc = "Bit 4 - Wdg reset req en"]
    #[inline(always)]
    pub fn wdg_reset_req_en(&mut self) -> WdgResetReqEnW<'_, RstCrSpec> {
        WdgResetReqEnW::new(self, 4)
    }
    #[doc = "Bit 5 - Iwdg reset req en"]
    #[inline(always)]
    pub fn iwdg_reset_req_en(&mut self) -> IwdgResetReqEnW<'_, RstCrSpec> {
        IwdgResetReqEnW::new(self, 5)
    }
}
#[doc = "reset control register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`rst_cr::R`](R). You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rst_cr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RstCrSpec;
impl crate::RegisterSpec for RstCrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rst_cr::R`](R) reader structure"]
impl crate::Readable for RstCrSpec {}
#[doc = "`write(|w| ..)` method takes [`rst_cr::W`](W) writer structure"]
impl crate::Writable for RstCrSpec {
    type Safety = crate::Unsafe;
}
