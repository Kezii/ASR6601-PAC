#[doc = "Register `SSP_DR` reader"]
pub type R = crate::R<SspDrSpec>;
#[doc = "Register `SSP_DR` writer"]
pub type W = crate::W<SspDrSpec>;
#[doc = "Field `DATA` reader - ssp tx/rx data"]
pub type DataR = crate::FieldReader<u16>;
#[doc = "Field `DATA` writer - ssp tx/rx data"]
pub type DataW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - ssp tx/rx data"]
    #[inline(always)]
    pub fn data(&self) -> DataR {
        DataR::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - ssp tx/rx data"]
    #[inline(always)]
    pub fn data(&mut self) -> DataW<'_, SspDrSpec> {
        DataW::new(self, 0)
    }
}
#[doc = "ssp data register\n\nYou can [`read`](crate::Reg::read) this register and get [`ssp_dr::R`](R). You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ssp_dr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SspDrSpec;
impl crate::RegisterSpec for SspDrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ssp_dr::R`](R) reader structure"]
impl crate::Readable for SspDrSpec {}
#[doc = "`write(|w| ..)` method takes [`ssp_dr::W`](W) writer structure"]
impl crate::Writable for SspDrSpec {
    type Safety = crate::Unsafe;
}
