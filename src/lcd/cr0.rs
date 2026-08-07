#[doc = "Register `CR0` reader"]
pub type R = crate::R<Cr0Spec>;
#[doc = "Register `CR0` writer"]
pub type W = crate::W<Cr0Spec>;
#[doc = "Field `DONE_STATUS` reader - Done status"]
pub type DoneStatusR = crate::BitReader;
#[doc = "Field `DONE_STATUS` writer - Done status"]
pub type DoneStatusW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Done status"]
    #[inline(always)]
    pub fn done_status(&self) -> DoneStatusR {
        DoneStatusR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Done status"]
    #[inline(always)]
    pub fn done_status(&mut self) -> DoneStatusW<'_, Cr0Spec> {
        DoneStatusW::new(self, 0)
    }
}
#[doc = "control register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`cr0::R`](R). You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cr0Spec;
impl crate::RegisterSpec for Cr0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cr0::R`](R) reader structure"]
impl crate::Readable for Cr0Spec {}
#[doc = "`write(|w| ..)` method takes [`cr0::W`](W) writer structure"]
impl crate::Writable for Cr0Spec {
    type Safety = crate::Unsafe;
}
