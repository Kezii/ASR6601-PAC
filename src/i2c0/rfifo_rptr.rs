#[doc = "Register `RFIFO_RPTR` reader"]
pub type R = crate::R<RfifoRptrSpec>;
#[doc = "Register `RFIFO_RPTR` writer"]
pub type W = crate::W<RfifoRptrSpec>;
#[doc = "Field `DATA` reader - position in the receive fifo where software will read the next entry"]
pub type DataR = crate::FieldReader;
#[doc = "Field `DATA` writer - position in the receive fifo where software will read the next entry"]
pub type DataW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - position in the receive fifo where software will read the next entry"]
    #[inline(always)]
    pub fn data(&self) -> DataR {
        DataR::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - position in the receive fifo where software will read the next entry"]
    #[inline(always)]
    pub fn data(&mut self) -> DataW<'_, RfifoRptrSpec> {
        DataW::new(self, 0)
    }
}
#[doc = "read fifo read pointer register\n\nYou can [`read`](crate::Reg::read) this register and get [`rfifo_rptr::R`](R). You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rfifo_rptr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RfifoRptrSpec;
impl crate::RegisterSpec for RfifoRptrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rfifo_rptr::R`](R) reader structure"]
impl crate::Readable for RfifoRptrSpec {}
#[doc = "`write(|w| ..)` method takes [`rfifo_rptr::W`](W) writer structure"]
impl crate::Writable for RfifoRptrSpec {
    type Safety = crate::Unsafe;
}
