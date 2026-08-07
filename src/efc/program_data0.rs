#[doc = "Register `PROGRAM_DATA0` reader"]
pub type R = crate::R<ProgramData0Spec>;
#[doc = "Register `PROGRAM_DATA0` writer"]
pub type W = crate::W<ProgramData0Spec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "program data0 register\n\nYou can [`read`](crate::Reg::read) this register and get [`program_data0::R`](R). You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`program_data0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ProgramData0Spec;
impl crate::RegisterSpec for ProgramData0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`program_data0::R`](R) reader structure"]
impl crate::Readable for ProgramData0Spec {}
#[doc = "`write(|w| ..)` method takes [`program_data0::W`](W) writer structure"]
impl crate::Writable for ProgramData0Spec {
    type Safety = crate::Unsafe;
}
