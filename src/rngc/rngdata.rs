#[doc = "Register `RNGDATA` reader"]
pub type R = crate::R<RngdataSpec>;
#[doc = "Register `RNGDATA` writer"]
pub type W = crate::W<RngdataSpec>;
#[doc = "Field `DATA` reader - Random data byte, valid per DATARDY; SDK reads the low byte"]
pub type DataR = crate::FieldReader;
#[doc = "Field `DATA` writer - Random data byte, valid per DATARDY; SDK reads the low byte"]
pub type DataW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Random data byte, valid per DATARDY; SDK reads the low byte"]
    #[inline(always)]
    pub fn data(&self) -> DataR {
        DataR::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Random data byte, valid per DATARDY; SDK reads the low byte"]
    #[inline(always)]
    pub fn data(&mut self) -> DataW<'_, RngdataSpec> {
        DataW::new(self, 0)
    }
}
#[doc = "Random data register\n\nYou can [`read`](crate::Reg::read) this register and get [`rngdata::R`](R). You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rngdata::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RngdataSpec;
impl crate::RegisterSpec for RngdataSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rngdata::R`](R) reader structure"]
impl crate::Readable for RngdataSpec {}
#[doc = "`write(|w| ..)` method takes [`rngdata::W`](W) writer structure"]
impl crate::Writable for RngdataSpec {
    type Safety = crate::Unsafe;
}
