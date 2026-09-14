#[doc = "Register `PROGRAM_DATA0` reader"]
pub type R = crate::R<ProgramData0Spec>;
#[doc = "Register `PROGRAM_DATA0` writer"]
pub type W = crate::W<ProgramData0Spec>;
#[doc = "Field `PROG_DATA0` reader - Program data 0"]
pub type ProgData0R = crate::FieldReader<u32>;
#[doc = "Field `PROG_DATA0` writer - Program data 0"]
pub type ProgData0W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Program data 0"]
    #[inline(always)]
    pub fn prog_data0(&self) -> ProgData0R {
        ProgData0R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Program data 0"]
    #[inline(always)]
    pub fn prog_data0(&mut self) -> ProgData0W<'_, ProgramData0Spec> {
        ProgData0W::new(self, 0)
    }
}
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
