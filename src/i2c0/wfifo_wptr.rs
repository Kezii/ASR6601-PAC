#[doc = "Register `WFIFO_WPTR` reader"]
pub type R = crate::R<WfifoWptrSpec>;
#[doc = "Register `WFIFO_WPTR` writer"]
pub type W = crate::W<WfifoWptrSpec>;
#[doc = "Field `DATA` reader - position in the transmit fifo where software will write the next entry"]
pub type DataR = crate::FieldReader;
#[doc = "Field `DATA` writer - position in the transmit fifo where software will write the next entry"]
pub type DataW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - position in the transmit fifo where software will write the next entry"]
    #[inline(always)]
    pub fn data(&self) -> DataR {
        DataR::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - position in the transmit fifo where software will write the next entry"]
    #[inline(always)]
    pub fn data(&mut self) -> DataW<'_, WfifoWptrSpec> {
        DataW::new(self, 0)
    }
}
#[doc = "write fifo write pointer register\n\nYou can [`read`](crate::Reg::read) this register and get [`wfifo_wptr::R`](R). You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wfifo_wptr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WfifoWptrSpec;
impl crate::RegisterSpec for WfifoWptrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`wfifo_wptr::R`](R) reader structure"]
impl crate::Readable for WfifoWptrSpec {}
#[doc = "`write(|w| ..)` method takes [`wfifo_wptr::W`](W) writer structure"]
impl crate::Writable for WfifoWptrSpec {
    type Safety = crate::Unsafe;
}
