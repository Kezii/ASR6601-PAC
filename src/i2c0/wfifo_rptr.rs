#[doc = "Register `WFIFO_RPTR` reader"]
pub type R = crate::R<WfifoRptrSpec>;
#[doc = "Register `WFIFO_RPTR` writer"]
pub type W = crate::W<WfifoRptrSpec>;
#[doc = "Field `DATA` reader - position in the transmit fifo where hardware will read the next entry"]
pub type DataR = crate::FieldReader;
#[doc = "Field `DATA` writer - position in the transmit fifo where hardware will read the next entry"]
pub type DataW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - position in the transmit fifo where hardware will read the next entry"]
    #[inline(always)]
    pub fn data(&self) -> DataR {
        DataR::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - position in the transmit fifo where hardware will read the next entry"]
    #[inline(always)]
    pub fn data(&mut self) -> DataW<'_, WfifoRptrSpec> {
        DataW::new(self, 0)
    }
}
#[doc = "write fifo read pointer register\n\nYou can [`read`](crate::Reg::read) this register and get [`wfifo_rptr::R`](R). You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wfifo_rptr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WfifoRptrSpec;
impl crate::RegisterSpec for WfifoRptrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`wfifo_rptr::R`](R) reader structure"]
impl crate::Readable for WfifoRptrSpec {}
#[doc = "`write(|w| ..)` method takes [`wfifo_rptr::W`](W) writer structure"]
impl crate::Writable for WfifoRptrSpec {
    type Safety = crate::Unsafe;
}
