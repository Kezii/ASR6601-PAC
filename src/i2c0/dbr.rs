#[doc = "Register `DBR` reader"]
pub type R = crate::R<DbrSpec>;
#[doc = "Register `DBR` writer"]
pub type W = crate::W<DbrSpec>;
#[doc = "Field `DATA_BUFFER` reader - i2c bus transmit/receive data"]
pub type DataBufferR = crate::FieldReader;
#[doc = "Field `DATA_BUFFER` writer - i2c bus transmit/receive data"]
pub type DataBufferW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - i2c bus transmit/receive data"]
    #[inline(always)]
    pub fn data_buffer(&self) -> DataBufferR {
        DataBufferR::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - i2c bus transmit/receive data"]
    #[inline(always)]
    pub fn data_buffer(&mut self) -> DataBufferW<'_, DbrSpec> {
        DataBufferW::new(self, 0)
    }
}
#[doc = "data buffer register\n\nYou can [`read`](crate::Reg::read) this register and get [`dbr::R`](R). You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dbr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DbrSpec;
impl crate::RegisterSpec for DbrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dbr::R`](R) reader structure"]
impl crate::Readable for DbrSpec {}
#[doc = "`write(|w| ..)` method takes [`dbr::W`](W) writer structure"]
impl crate::Writable for DbrSpec {
    type Safety = crate::Unsafe;
}
