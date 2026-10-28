#[doc = "Register `OR` reader"]
pub type R = crate::R<OrSpec>;
#[doc = "Register `OR` writer"]
pub type W = crate::W<OrSpec>;
#[doc = "Channel 0 remapping"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Ti0Rmp {
    #[doc = "0: IOM"]
    Iom = 0,
    #[doc = "1: comp0"]
    Comp0 = 1,
    #[doc = "2: comp1"]
    Comp1 = 2,
    #[doc = "3: uart_rx\\[0\\]"]
    UartRx0 = 3,
    #[doc = "4: uart_rx\\[1\\]"]
    UartRx1 = 4,
    #[doc = "5: uart_rx\\[2\\]"]
    UartRx2 = 5,
    #[doc = "6: uart_rx\\[3\\]"]
    UartRx3 = 6,
    #[doc = "7: uart_rx\\[4\\]"]
    UartRx4 = 7,
}
impl From<Ti0Rmp> for u8 {
    #[inline(always)]
    fn from(variant: Ti0Rmp) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Ti0Rmp {
    type Ux = u8;
}
impl crate::IsEnum for Ti0Rmp {}
#[doc = "Field `TI0_RMP` reader - Channel 0 remapping"]
pub type Ti0RmpR = crate::FieldReader<Ti0Rmp>;
impl Ti0RmpR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ti0Rmp {
        match self.bits {
            0 => Ti0Rmp::Iom,
            1 => Ti0Rmp::Comp0,
            2 => Ti0Rmp::Comp1,
            3 => Ti0Rmp::UartRx0,
            4 => Ti0Rmp::UartRx1,
            5 => Ti0Rmp::UartRx2,
            6 => Ti0Rmp::UartRx3,
            7 => Ti0Rmp::UartRx4,
            _ => unreachable!(),
        }
    }
    #[doc = "IOM"]
    #[inline(always)]
    pub fn is_iom(&self) -> bool {
        *self == Ti0Rmp::Iom
    }
    #[doc = "comp0"]
    #[inline(always)]
    pub fn is_comp0(&self) -> bool {
        *self == Ti0Rmp::Comp0
    }
    #[doc = "comp1"]
    #[inline(always)]
    pub fn is_comp1(&self) -> bool {
        *self == Ti0Rmp::Comp1
    }
    #[doc = "uart_rx\\[0\\]"]
    #[inline(always)]
    pub fn is_uart_rx_0(&self) -> bool {
        *self == Ti0Rmp::UartRx0
    }
    #[doc = "uart_rx\\[1\\]"]
    #[inline(always)]
    pub fn is_uart_rx_1(&self) -> bool {
        *self == Ti0Rmp::UartRx1
    }
    #[doc = "uart_rx\\[2\\]"]
    #[inline(always)]
    pub fn is_uart_rx_2(&self) -> bool {
        *self == Ti0Rmp::UartRx2
    }
    #[doc = "uart_rx\\[3\\]"]
    #[inline(always)]
    pub fn is_uart_rx_3(&self) -> bool {
        *self == Ti0Rmp::UartRx3
    }
    #[doc = "uart_rx\\[4\\]"]
    #[inline(always)]
    pub fn is_uart_rx_4(&self) -> bool {
        *self == Ti0Rmp::UartRx4
    }
}
#[doc = "Field `TI0_RMP` writer - Channel 0 remapping"]
pub type Ti0RmpW<'a, REG> = crate::FieldWriter<'a, REG, 3, Ti0Rmp, crate::Safe>;
impl<'a, REG> Ti0RmpW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "IOM"]
    #[inline(always)]
    pub fn iom(self) -> &'a mut crate::W<REG> {
        self.variant(Ti0Rmp::Iom)
    }
    #[doc = "comp0"]
    #[inline(always)]
    pub fn comp0(self) -> &'a mut crate::W<REG> {
        self.variant(Ti0Rmp::Comp0)
    }
    #[doc = "comp1"]
    #[inline(always)]
    pub fn comp1(self) -> &'a mut crate::W<REG> {
        self.variant(Ti0Rmp::Comp1)
    }
    #[doc = "uart_rx\\[0\\]"]
    #[inline(always)]
    pub fn uart_rx_0(self) -> &'a mut crate::W<REG> {
        self.variant(Ti0Rmp::UartRx0)
    }
    #[doc = "uart_rx\\[1\\]"]
    #[inline(always)]
    pub fn uart_rx_1(self) -> &'a mut crate::W<REG> {
        self.variant(Ti0Rmp::UartRx1)
    }
    #[doc = "uart_rx\\[2\\]"]
    #[inline(always)]
    pub fn uart_rx_2(self) -> &'a mut crate::W<REG> {
        self.variant(Ti0Rmp::UartRx2)
    }
    #[doc = "uart_rx\\[3\\]"]
    #[inline(always)]
    pub fn uart_rx_3(self) -> &'a mut crate::W<REG> {
        self.variant(Ti0Rmp::UartRx3)
    }
    #[doc = "uart_rx\\[4\\]"]
    #[inline(always)]
    pub fn uart_rx_4(self) -> &'a mut crate::W<REG> {
        self.variant(Ti0Rmp::UartRx4)
    }
}
#[doc = "ETR remapping"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum EtrRmp {
    #[doc = "0: IOM"]
    Iom = 0,
    #[doc = "1: comp0"]
    Comp0 = 1,
    #[doc = "2: comp1"]
    Comp1 = 2,
    #[doc = "3: xo32k"]
    Xo32k = 3,
    #[doc = "4: uart_rx\\[0\\]"]
    UartRx0 = 4,
    #[doc = "5: uart_rx\\[1\\]"]
    UartRx1 = 5,
    #[doc = "6: uart_rx\\[2\\]"]
    UartRx2 = 6,
    #[doc = "7: uart_rx\\[3\\]"]
    UartRx3 = 7,
    #[doc = "8: uart_rx\\[4\\]"]
    UartRx4 = 8,
}
impl From<EtrRmp> for u8 {
    #[inline(always)]
    fn from(variant: EtrRmp) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for EtrRmp {
    type Ux = u8;
}
impl crate::IsEnum for EtrRmp {}
#[doc = "Field `ETR_RMP` reader - ETR remapping"]
pub type EtrRmpR = crate::FieldReader<EtrRmp>;
impl EtrRmpR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<EtrRmp> {
        match self.bits {
            0 => Some(EtrRmp::Iom),
            1 => Some(EtrRmp::Comp0),
            2 => Some(EtrRmp::Comp1),
            3 => Some(EtrRmp::Xo32k),
            4 => Some(EtrRmp::UartRx0),
            5 => Some(EtrRmp::UartRx1),
            6 => Some(EtrRmp::UartRx2),
            7 => Some(EtrRmp::UartRx3),
            8 => Some(EtrRmp::UartRx4),
            _ => None,
        }
    }
    #[doc = "IOM"]
    #[inline(always)]
    pub fn is_iom(&self) -> bool {
        *self == EtrRmp::Iom
    }
    #[doc = "comp0"]
    #[inline(always)]
    pub fn is_comp0(&self) -> bool {
        *self == EtrRmp::Comp0
    }
    #[doc = "comp1"]
    #[inline(always)]
    pub fn is_comp1(&self) -> bool {
        *self == EtrRmp::Comp1
    }
    #[doc = "xo32k"]
    #[inline(always)]
    pub fn is_xo32k(&self) -> bool {
        *self == EtrRmp::Xo32k
    }
    #[doc = "uart_rx\\[0\\]"]
    #[inline(always)]
    pub fn is_uart_rx_0(&self) -> bool {
        *self == EtrRmp::UartRx0
    }
    #[doc = "uart_rx\\[1\\]"]
    #[inline(always)]
    pub fn is_uart_rx_1(&self) -> bool {
        *self == EtrRmp::UartRx1
    }
    #[doc = "uart_rx\\[2\\]"]
    #[inline(always)]
    pub fn is_uart_rx_2(&self) -> bool {
        *self == EtrRmp::UartRx2
    }
    #[doc = "uart_rx\\[3\\]"]
    #[inline(always)]
    pub fn is_uart_rx_3(&self) -> bool {
        *self == EtrRmp::UartRx3
    }
    #[doc = "uart_rx\\[4\\]"]
    #[inline(always)]
    pub fn is_uart_rx_4(&self) -> bool {
        *self == EtrRmp::UartRx4
    }
}
#[doc = "Field `ETR_RMP` writer - ETR remapping"]
pub type EtrRmpW<'a, REG> = crate::FieldWriter<'a, REG, 4, EtrRmp>;
impl<'a, REG> EtrRmpW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "IOM"]
    #[inline(always)]
    pub fn iom(self) -> &'a mut crate::W<REG> {
        self.variant(EtrRmp::Iom)
    }
    #[doc = "comp0"]
    #[inline(always)]
    pub fn comp0(self) -> &'a mut crate::W<REG> {
        self.variant(EtrRmp::Comp0)
    }
    #[doc = "comp1"]
    #[inline(always)]
    pub fn comp1(self) -> &'a mut crate::W<REG> {
        self.variant(EtrRmp::Comp1)
    }
    #[doc = "xo32k"]
    #[inline(always)]
    pub fn xo32k(self) -> &'a mut crate::W<REG> {
        self.variant(EtrRmp::Xo32k)
    }
    #[doc = "uart_rx\\[0\\]"]
    #[inline(always)]
    pub fn uart_rx_0(self) -> &'a mut crate::W<REG> {
        self.variant(EtrRmp::UartRx0)
    }
    #[doc = "uart_rx\\[1\\]"]
    #[inline(always)]
    pub fn uart_rx_1(self) -> &'a mut crate::W<REG> {
        self.variant(EtrRmp::UartRx1)
    }
    #[doc = "uart_rx\\[2\\]"]
    #[inline(always)]
    pub fn uart_rx_2(self) -> &'a mut crate::W<REG> {
        self.variant(EtrRmp::UartRx2)
    }
    #[doc = "uart_rx\\[3\\]"]
    #[inline(always)]
    pub fn uart_rx_3(self) -> &'a mut crate::W<REG> {
        self.variant(EtrRmp::UartRx3)
    }
    #[doc = "uart_rx\\[4\\]"]
    #[inline(always)]
    pub fn uart_rx_4(self) -> &'a mut crate::W<REG> {
        self.variant(EtrRmp::UartRx4)
    }
}
impl R {
    #[doc = "Bits 0:2 - Channel 0 remapping"]
    #[inline(always)]
    pub fn ti0_rmp(&self) -> Ti0RmpR {
        Ti0RmpR::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 3:6 - ETR remapping"]
    #[inline(always)]
    pub fn etr_rmp(&self) -> EtrRmpR {
        EtrRmpR::new(((self.bits >> 3) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Channel 0 remapping"]
    #[inline(always)]
    pub fn ti0_rmp(&mut self) -> Ti0RmpW<'_, OrSpec> {
        Ti0RmpW::new(self, 0)
    }
    #[doc = "Bits 3:6 - ETR remapping"]
    #[inline(always)]
    pub fn etr_rmp(&mut self) -> EtrRmpW<'_, OrSpec> {
        EtrRmpW::new(self, 3)
    }
}
#[doc = "GPTIM option register (GPTIM3 remap layout)\n\nYou can [`read`](crate::Reg::read) this register and get [`or::R`](R). You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`or::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OrSpec;
impl crate::RegisterSpec for OrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`or::R`](R) reader structure"]
impl crate::Readable for OrSpec {}
#[doc = "`write(|w| ..)` method takes [`or::W`](W) writer structure"]
impl crate::Writable for OrSpec {
    type Safety = crate::Unsafe;
}
