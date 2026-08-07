#[doc = "Register `RST_SR` reader"]
pub type R = crate::R<RstSrSpec>;
#[doc = "Register `RST_SR` writer"]
pub type W = crate::W<RstSrSpec>;
#[doc = "Field `STANDBY_RESET_SR` reader - Standby reset sr"]
pub type StandbyResetSrR = crate::BitReader;
#[doc = "Field `STANDBY_RESET_SR` writer - Standby reset sr"]
pub type StandbyResetSrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SEC_RESET_SR` reader - Sec reset sr"]
pub type SecResetSrR = crate::BitReader;
#[doc = "Field `SEC_RESET_SR` writer - Sec reset sr"]
pub type SecResetSrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CPU_RESET_SR` reader - Cpu reset sr"]
pub type CpuResetSrR = crate::BitReader;
#[doc = "Field `CPU_RESET_SR` writer - Cpu reset sr"]
pub type CpuResetSrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EFC_RESET_SR` reader - Efc reset sr"]
pub type EfcResetSrR = crate::BitReader;
#[doc = "Field `EFC_RESET_SR` writer - Efc reset sr"]
pub type EfcResetSrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WDG_RESET_SR` reader - Wdg reset sr"]
pub type WdgResetSrR = crate::BitReader;
#[doc = "Field `WDG_RESET_SR` writer - Wdg reset sr"]
pub type WdgResetSrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IWDG_RESET_SR` reader - Iwdg reset sr"]
pub type IwdgResetSrR = crate::BitReader;
#[doc = "Field `IWDG_RESET_SR` writer - Iwdg reset sr"]
pub type IwdgResetSrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BOR_RESET_SR` reader - Bor reset sr"]
pub type BorResetSrR = crate::BitReader;
#[doc = "Field `BOR_RESET_SR` writer - Bor reset sr"]
pub type BorResetSrW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Standby reset sr"]
    #[inline(always)]
    pub fn standby_reset_sr(&self) -> StandbyResetSrR {
        StandbyResetSrR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Sec reset sr"]
    #[inline(always)]
    pub fn sec_reset_sr(&self) -> SecResetSrR {
        SecResetSrR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Cpu reset sr"]
    #[inline(always)]
    pub fn cpu_reset_sr(&self) -> CpuResetSrR {
        CpuResetSrR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Efc reset sr"]
    #[inline(always)]
    pub fn efc_reset_sr(&self) -> EfcResetSrR {
        EfcResetSrR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Wdg reset sr"]
    #[inline(always)]
    pub fn wdg_reset_sr(&self) -> WdgResetSrR {
        WdgResetSrR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Iwdg reset sr"]
    #[inline(always)]
    pub fn iwdg_reset_sr(&self) -> IwdgResetSrR {
        IwdgResetSrR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Bor reset sr"]
    #[inline(always)]
    pub fn bor_reset_sr(&self) -> BorResetSrR {
        BorResetSrR::new(((self.bits >> 6) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Standby reset sr"]
    #[inline(always)]
    pub fn standby_reset_sr(&mut self) -> StandbyResetSrW<'_, RstSrSpec> {
        StandbyResetSrW::new(self, 0)
    }
    #[doc = "Bit 1 - Sec reset sr"]
    #[inline(always)]
    pub fn sec_reset_sr(&mut self) -> SecResetSrW<'_, RstSrSpec> {
        SecResetSrW::new(self, 1)
    }
    #[doc = "Bit 2 - Cpu reset sr"]
    #[inline(always)]
    pub fn cpu_reset_sr(&mut self) -> CpuResetSrW<'_, RstSrSpec> {
        CpuResetSrW::new(self, 2)
    }
    #[doc = "Bit 3 - Efc reset sr"]
    #[inline(always)]
    pub fn efc_reset_sr(&mut self) -> EfcResetSrW<'_, RstSrSpec> {
        EfcResetSrW::new(self, 3)
    }
    #[doc = "Bit 4 - Wdg reset sr"]
    #[inline(always)]
    pub fn wdg_reset_sr(&mut self) -> WdgResetSrW<'_, RstSrSpec> {
        WdgResetSrW::new(self, 4)
    }
    #[doc = "Bit 5 - Iwdg reset sr"]
    #[inline(always)]
    pub fn iwdg_reset_sr(&mut self) -> IwdgResetSrW<'_, RstSrSpec> {
        IwdgResetSrW::new(self, 5)
    }
    #[doc = "Bit 6 - Bor reset sr"]
    #[inline(always)]
    pub fn bor_reset_sr(&mut self) -> BorResetSrW<'_, RstSrSpec> {
        BorResetSrW::new(self, 6)
    }
}
#[doc = "reset status register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`rst_sr::R`](R). You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rst_sr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RstSrSpec;
impl crate::RegisterSpec for RstSrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rst_sr::R`](R) reader structure"]
impl crate::Readable for RstSrSpec {}
#[doc = "`write(|w| ..)` method takes [`rst_sr::W`](W) writer structure"]
impl crate::Writable for RstSrSpec {
    type Safety = crate::Unsafe;
}
