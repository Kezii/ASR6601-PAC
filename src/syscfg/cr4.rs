#[doc = "Register `CR4` reader"]
pub type R = crate::R<Cr4Spec>;
#[doc = "Register `CR4` writer"]
pub type W = crate::W<Cr4Spec>;
#[doc = "Field `SYSCFG_CR4_REG` reader - lptim1 in2 remapping enable"]
pub type SyscfgCr4RegR = crate::BitReader;
#[doc = "Field `SYSCFG_CR4_REG` writer - lptim1 in2 remapping enable"]
pub type SyscfgCr4RegW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 31 - lptim1 in2 remapping enable"]
    #[inline(always)]
    pub fn syscfg_cr4_reg(&self) -> SyscfgCr4RegR {
        SyscfgCr4RegR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 31 - lptim1 in2 remapping enable"]
    #[inline(always)]
    pub fn syscfg_cr4_reg(&mut self) -> SyscfgCr4RegW<'_, Cr4Spec> {
        SyscfgCr4RegW::new(self, 31)
    }
}
#[doc = "control register 4\n\nYou can [`read`](crate::Reg::read) this register and get [`cr4::R`](R). You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr4::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cr4Spec;
impl crate::RegisterSpec for Cr4Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cr4::R`](R) reader structure"]
impl crate::Readable for Cr4Spec {}
#[doc = "`write(|w| ..)` method takes [`cr4::W`](W) writer structure"]
impl crate::Writable for Cr4Spec {
    type Safety = crate::Unsafe;
}
